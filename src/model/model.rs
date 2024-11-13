use pyo3::{pyclass, pymethods, Bound, IntoPy, PyAny, PyObject, PyResult, Python};
use teo::prelude::model;

use crate::{dynamic::py_class_lookup_map::PYClassLookupMap, object::value::{py_any_to_teo_value, teo_value_to_py_any}};

#[pyclass]
pub struct Model {
    pub(crate) teo_model: model::Builder,
}

#[pymethods]
impl Model {

    pub fn set_data(&mut self, py: Python<'_>, key: String, value: &Bound<PyAny>) -> PyResult<()> {
        self.teo_model.data().insert(key, py_any_to_teo_value(py, value)?);
        Ok(())
    }

    pub fn data(&mut self, py: Python<'_>, key: String) -> PyResult<PyObject> {
        Ok(match self.teo_model.data().get(key.as_str()) {
            Some(object) => teo_value_to_py_any(py, object, PYClassLookupMap::from_app_data(self.teo_model.app_data()))?,
            None => ().into_py(py),
        })
    }
}