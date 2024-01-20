use pyo3::{Python, PyResult, PyObject};
use teo::prelude::model;

//use crate::dynamic::js_model_object_from_teo_model_object;

pub fn teo_model_object_to_py_any(py: Python<'_>, model_object: &model::Object) -> PyResult<PyObject> {
    // let js_object = js_model_object_from_teo_model_object(*env, model_object.clone())?;
    // Ok(js_object.into_unknown())
    unreachable!()
}