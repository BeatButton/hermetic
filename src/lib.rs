use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

#[pyfunction]
/// Placeholder
fn placeholder() -> PyResult<usize> {
    Ok(1)
}

/// This module is a python module implemented in Rust.
#[pymodule]
#[allow(unused)]
fn hermetic(py: Python, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(placeholder))?;

    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
