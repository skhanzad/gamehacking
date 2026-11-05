use pyo3::prelude::*;


#[pyfunction]
fn write_memory(pid: i32, address: usize, value: i32) -> PyResult<()> {
    // Implement the memory writing logic here
    Ok(())
}