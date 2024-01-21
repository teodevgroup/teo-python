use teo::prelude::transaction::Ctx;
use pyo3::prelude::*;

#[pyclass]
#[derive(Clone)]
pub struct TransactionCtxWrapper {
    pub ctx: Ctx
}

impl TransactionCtxWrapper {
    pub fn new(ctx: Ctx) -> Self {
        Self { ctx }
    }
}