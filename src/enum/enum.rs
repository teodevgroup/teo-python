use pyo3::{pyclass, pymethods, types::PyAnyMethods, Bound, IntoPy, PyAny, PyObject, PyResult, Python};
use teo::prelude::r#enum;

use crate::object::value::{py_any_to_teo_value, teo_value_to_py_any};

#[pyclass]
pub struct Enum {
    pub(crate) teo_enum: r#enum::Builder,
}

#[pymethods]
impl Enum {

    pub fn set_data(&mut self, py: Python<'_>, key: String, value: &Bound<PyAny>) -> PyResult<()> {
        self.teo_enum.data().insert(key, py_any_to_teo_value(py, value.extract::<&PyAny>()?)?);
        Ok(())
    }

    pub fn data(&mut self, py: Python<'_>, key: String) -> PyResult<PyObject> {
        Ok(match self.teo_enum.data().get(key.as_str()) {
            Some(object) => teo_value_to_py_any(py, object)?,
            None => ().into_py(py),
        })
    }
}