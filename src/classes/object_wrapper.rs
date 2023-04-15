use ::teo::prelude::Object;
use pyo3::prelude::*;

#[pyclass]
pub struct ObjectWrapper {
    pub object: Object
}

impl ObjectWrapper {
    pub fn new(object: Object) -> Self {
        Self { object }
    }
}