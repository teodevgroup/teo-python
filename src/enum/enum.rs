use pyo3::{PyResult, Python, pyclass, pymethods, IntoPy, PyObject};
use teo::prelude::Enum as TeoEnum;

use crate::object::{py_any_to_teo_object, teo_object_to_py_any};

#[pyclass]
pub struct Enum {
    pub(crate) teo_enum: &'static mut TeoEnum,
}

#[pymethods]
impl Enum {

    pub fn set_data(&mut self, py: Python<'_>, key: String, value: PyObject) -> PyResult<()> {
        self.teo_enum.data.insert(key, py_any_to_teo_object(py, value)?);
        Ok(())
    }

    pub fn data(&mut self, py: Python<'_>, key: String) -> PyResult<PyObject> {
        Ok(match self.teo_enum.data.get(key.as_str()) {
            Some(object) => teo_object_to_py_any(py, object)?,
            None => ().into_py(py),
        })
    }
}