use teo::prelude::model::Object;
use pyo3::prelude::*;

#[pyclass]
#[derive(Clone)]
pub struct ModelObjectWrapper {
    pub object: Object
}

impl ModelObjectWrapper {
    pub fn new(object: Object) -> Self {
        Self { object }
    }
}