use pyo3::exceptions::PyOSError;
use pyo3::prelude::*;
use std::fs;

#[derive(Clone, Debug)]
pub struct Region { pub start: usize, pub end: usize, pub perms: String }

#[pyclass]
pub struct Process { #[pyo3(get)] pub pid: i32 }

fn os_err(e: impl std::fmt::Display) -> PyErr { PyOSError::new_err(e.to_string()) }

impl Process {
    pub fn regions_internal(&self) -> std::io::Result<Vec<Region>> {
        let maps = fs::read_to_string(format!("/proc/{}/maps", self.pid))?;
        Ok(maps.lines().filter_map(|l| {
            let mut it = l.split_whitespace();
            let (range, perms) = (it.next()?, it.next()?);
            let (s, e) = range.split_once('-')?;
            Some(Region {
                start: usize::from_str_radix(s, 16).ok()?,
                end: usize::from_str_radix(e, 16).ok()?,
                perms: perms.to_string(),
            })
        }).collect())
    }

    #[cfg(target_os = "linux")]
    pub fn read_internal(&self, addr: usize, size: usize) -> std::io::Result<Vec<u8>> {
        let mut buf = vec![0u8; size];
        let local = libc::iovec { iov_base: buf.as_mut_ptr() as *mut _, iov_len: size };
        let remote = libc::iovec { iov_base: addr as *mut _, iov_len: size };
        let n = unsafe { libc::process_vm_readv(self.pid, &local, 1, &remote, 1, 0) };
        if n < 0 { return Err(std::io::Error::last_os_error()); }
        buf.truncate(n as usize);
        Ok(buf)
    }
}

#[pymethods]
impl Process {
    #[new]
    fn new(pid: i32) -> Self { Self { pid } }

    /// List memory regions as (start, end, perms).
    fn regions(&self) -> PyResult<Vec<(usize, usize, String)>> {
        Ok(self.regions_internal().map_err(os_err)?
            .into_iter().map(|r| (r.start, r.end, r.perms)).collect())
    }

    fn read(&self, addr: usize, size: usize) -> PyResult<Vec<u8>> {
        self.read_internal(addr, size).map_err(os_err)
    }

    #[cfg(target_os = "linux")]
    fn write(&self, addr: usize, data: Vec<u8>) -> PyResult<usize> {
        let local = libc::iovec { iov_base: data.as_ptr() as *mut _, iov_len: data.len() };
        let remote = libc::iovec { iov_base: addr as *mut _, iov_len: data.len() };
        let n = unsafe { libc::process_vm_writev(self.pid, &local, 1, &remote, 1, 0) };
        if n < 0 { return Err(os_err(std::io::Error::last_os_error())); }
        Ok(n as usize)
    }

    /// Scan all writable regions for an exact byte pattern. Returns addresses.
    fn scan(&self, pattern: Vec<u8>) -> PyResult<Vec<usize>> {
        use rayon::prelude::*;
        let regions: Vec<_> = self.regions_internal().map_err(os_err)?
            .into_iter().filter(|r| r.perms.starts_with("rw")).collect();
        // Read every region; some (e.g. guard pages, [vvar]) legitimately fail.
        let results: Vec<_> = regions.par_iter()
            .map(|r| self.read_internal(r.start, r.end - r.start)
                .map(|buf| crate::scan::find_all(&buf, &pattern).into_iter()
                    .map(|o| r.start + o).collect::<Vec<_>>()))
            .collect();

        // Permission errors mean we can't read the process at all: report it
        // instead of silently returning zero matches.
        if let Some(Err(e)) = results.iter().find(|r| matches!(r,
            Err(e) if e.raw_os_error() == Some(libc::EPERM)))
        {
            return Err(PyOSError::new_err(format!(
                "cannot read memory of pid {}: {e}. Allow it with \
                 `sudo sysctl kernel.yama.ptrace_scope=0` or run as root", self.pid)));
        }
        Ok(results.into_iter().filter_map(Result::ok).flatten().collect())
    }
}
