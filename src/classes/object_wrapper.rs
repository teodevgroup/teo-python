use ::teo::prelude::Object;
use pyo3::prelude::*;

#[pyclass]
pub struct ObjectWrapper {
    object: Object
}