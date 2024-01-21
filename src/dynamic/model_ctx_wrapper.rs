use teo::prelude::model::Ctx;
use pyo3::prelude::*;

#[pyclass]
#[derive(Clone)]
pub struct ModelCtxWrapper {
    pub ctx: Ctx
}

impl ModelCtxWrapper {
    pub fn new(ctx: Ctx) -> Self {
        Self { ctx }
    }
}