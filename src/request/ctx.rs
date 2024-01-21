use pyo3::{pyclass, pymethods, ffi::PyObject, PyResult, Python};
use teo::prelude::request::Ctx as TeoRequestCtx;
use crate::{dynamic::js_ctx_object_from_teo_transaction_ctx, object::value::teo_value_to_py_any};
use crate::object::value::teo_value_to_js_any;

use super::{Request, HandlerMatch};

#[pyclass]
pub struct RequestCtx {
    teo_inner: TeoRequestCtx,
}

/// HTTP request.
#[pymethods]
impl RequestCtx {

    pub(crate) fn new(teo_inner: TeoRequestCtx) -> Self {
        Self {
            teo_inner
        }
    }

    pub fn request(&self) -> Request {
        Request {
            teo_request: self.teo_inner.request().clone()
        }
    }

    pub fn body(&self, py: Python<'_>) -> PyResult<PyObject> {
        teo_value_to_py_any(py, self.teo_inner.body())
    }

    pub fn teo(&self) -> PyResult<PyObject> {
        Ok(js_ctx_object_from_teo_transaction_ctx(self.teo_inner.transaction_ctx(), "")?.into_unknown())
    }

    pub fn handler_match(&'static self) -> HandlerMatch {
        HandlerMatch::new(self.teo_inner.handler_match())
    }
}
