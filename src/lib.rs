use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

#[pyclass]
struct Placeholder {
    #[pyo3(get)]
    value: usize,
}

#[pymethods]
#[allow(clippy::new_ret_no_self)]
impl Placeholder {
    #[new]
    fn new(obj: &PyRawObject) {
        obj.init({ Placeholder { value: 3 } });
    }

    fn placeholder(&self) -> usize {
        Python::acquire_gil().python().allow_threads(|| 2)
    }
}

#[pyfunction]
/// Placeholder
fn placeholder() -> PyResult<usize> {
    Ok(1)
}

/// This module is a python module implemented in Rust.
#[pymodule]
fn hermetic(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(placeholder))?;
    m.add_class::<Placeholder>()?;

    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
