//! Enumerate running processes (Linux, via /proc).
use pyo3::prelude::*;
use std::fs;

#[pyclass(get_all)]
#[derive(Clone, Debug)]
pub struct ProcessInfo {
    pub pid: i32,
    pub name: String,
    pub cmdline: String,
}

#[pymethods]
impl ProcessInfo {
    fn __repr__(&self) -> String {
        format!("ProcessInfo(pid={}, name={:?})", self.pid, self.name)
    }
}

pub fn list_internal() -> std::io::Result<Vec<ProcessInfo>> {
    let mut out: Vec<ProcessInfo> = fs::read_dir("/proc")?
        .filter_map(Result::ok)
        .filter_map(|e| {
            let pid: i32 = e.file_name().to_str()?.parse().ok()?;
            // Process may exit between listing and reading; skip it then.
            let name = fs::read_to_string(e.path().join("comm")).ok()?.trim_end().to_string();
            let cmdline = fs::read(e.path().join("cmdline"))
                .map(|b| String::from_utf8_lossy(&b).split('\0')
                    .filter(|s| !s.is_empty()).collect::<Vec<_>>().join(" "))
                .unwrap_or_default();
            Some(ProcessInfo { pid, name, cmdline })
        })
        .collect();
    out.sort_by_key(|p| p.pid);
    Ok(out)
}

/// List running processes. If `filter` is given, keep only those whose
/// name or command line contains it (case-insensitive).
#[pyfunction]
#[pyo3(signature = (filter=None))]
pub fn list_processes(filter: Option<&str>) -> PyResult<Vec<ProcessInfo>> {
    let procs = list_internal().map_err(|e| pyo3::exceptions::PyOSError::new_err(e.to_string()))?;
    Ok(match filter {
        Some(f) => {
            let f = f.to_lowercase();
            procs.into_iter().filter(|p| {
                p.name.to_lowercase().contains(&f) || p.cmdline.to_lowercase().contains(&f)
            }).collect()
        }
        None => procs,
    })
}

#[cfg(test)]
mod tests {
    #[test]
    fn finds_self() {
        let me = std::process::id() as i32;
        assert!(super::list_internal().unwrap().iter().any(|p| p.pid == me));
    }
}
