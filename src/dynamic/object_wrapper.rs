use teo::prelude::model::Object;
use pyo3::prelude::*;

#[pyclass]
#[derive(Clone)]
pub struct ObjectWrapper {
    pub object: Object
}

impl ObjectWrapper {
    pub fn new(object: Object) -> Self {
        Self { object }
    }
}