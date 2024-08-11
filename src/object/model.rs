use pyo3::{Python, PyResult, PyObject};
use teo::prelude::{app::data::AppData, model};

use crate::dynamic::py_class_lookup_map::PYClassLookupMap;

pub fn teo_model_object_to_py_any(py: Python<'_>, model_object: &model::Object, app_data: &AppData) -> PyResult<PyObject> {
    let map = PYClassLookupMap::from_app_data(app_data);
    map.teo_model_object_to_py_model_object_object(py, model_object.clone())
}