//! Rust core: fast process memory access + scanning, exposed to Python.
mod memory;
mod scan;

use pyo3::prelude::*;

#[pymodule]
fn _core(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<memory::Process>()?;
    m.add_function(wrap_pyfunction!(scan::scan_bytes, m)?)?;
    Ok(())
}
