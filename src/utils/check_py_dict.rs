use pyo3::{Bound, PyAny, PyResult};
use pyo3::exceptions::PyValueError;
use pyo3::types::{PyAnyMethods, PyDict};

pub fn check_py_dict(dict: &Bound<PyAny>) -> PyResult<()> {
    if !dict.is_instance_of::<PyDict>() {
        return Err(PyValueError::new_err("argument is not dict"));
    }
    Ok(())
}