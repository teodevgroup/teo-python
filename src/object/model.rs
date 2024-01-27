use pyo3::{Python, PyResult, PyObject};
use teo::prelude::model;

use crate::dynamic::py_model_object_from_teo_model_object;

pub fn teo_model_object_to_py_any(py: Python<'_>, model_object: &model::Object) -> PyResult<PyObject> {
    py_model_object_from_teo_model_object(py, model_object.clone())
}