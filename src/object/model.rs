use pyo3::{Python, PyResult, PyObject};
use teo::prelude::model;
use crate::dynamic::{DynamicClasses, QueryDynamicClasses};

pub(crate) fn teo_model_object_to_py_any(py: Python<'_>, model_object: &model::Object, map: &DynamicClasses) -> PyResult<PyObject> {
    map.teo_model_object_to_py_model_object_object(py, model_object.clone())
}