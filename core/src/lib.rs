//! Rust core: fast process memory access + scanning, exposed to Python.
mod memory;
mod scan;
mod list_processes;
mod write;

use pyo3::prelude::*;

#[pymodule]
fn _core(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<memory::Process>()?;
    m.add_function(wrap_pyfunction!(scan::scan_bytes, m)?)?;
    m.add_class::<list_processes::ProcessInfo>()?;
    m.add_function(wrap_pyfunction!(list_processes::list_processes, m)?)?;
    m.add_function(wrap_pyfunction!(write::write_memory, m)?)?;
    Ok(())
}
