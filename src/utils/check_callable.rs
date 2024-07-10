use pyo3::types::PyAnyMethods;
use pyo3::{Bound, PyAny, PyResult};
use pyo3::exceptions::PyValueError;

pub fn check_callable(callback: &Bound<PyAny>) -> PyResult<()> {
    if !callback.is_callable() {
        return Err(PyValueError::new_err("parameter is not callable"));
    }
    Ok(())
}