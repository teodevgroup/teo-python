use pyo3::{PyResult, Python, pyclass, pymethods, IntoPy, PyObject};
use teo::prelude::Model as TeoModel;

use crate::object::{teo_object_to_py_any, py_any_to_teo_object};

#[pyclass]
pub struct Model {
    pub(crate) teo_model: &'static mut TeoModel,
}

#[pymethods]
impl Model {

    pub fn set_data(&mut self, py: Python<'_>, key: String, value: PyObject) -> PyResult<()> {
        self.teo_model.data.insert(key, py_any_to_teo_object(py, value)?);
        Ok(())
    }

    pub fn data(&mut self, py: Python<'_>, key: String) -> PyResult<PyObject> {
        Ok(match self.teo_model.data.get(key.as_str()) {
            Some(object) => teo_object_to_py_any(py, object)?,
            None => ().into_py(py),
        })
    }
}