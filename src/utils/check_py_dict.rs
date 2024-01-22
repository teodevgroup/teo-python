use pyo3::{PyAny, PyResult};
use pyo3::exceptions::PyValueError;
use pyo3::types::PyDict;

pub fn check_py_dict(dict: &PyAny) -> PyResult<()> {
    if !dict.is_instance_of::<PyDict>() {
        return Err(PyValueError::new_err("argument is not dict"));
    }
    Ok(())
}