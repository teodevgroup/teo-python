use pyo3::{Python, PyResult, PyObject};
use teo::prelude::model;

use crate::dynamic::py_class_lookup_map::PYClassLookupMap;

pub fn teo_model_object_to_py_any(py: Python<'_>, model_object: &model::Object, map: &PYClassLookupMap) -> PyResult<PyObject> {
    map.teo_model_object_to_py_model_object_object(py, model_object.clone())
}