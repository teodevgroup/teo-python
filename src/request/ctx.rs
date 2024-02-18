use pyo3::{pyclass, pymethods, PyObject, PyResult, Python};
use teo::prelude::request::Ctx as TeoRequestCtx;
use crate::{object::value::teo_value_to_py_any, dynamic::py_ctx_object_from_teo_transaction_ctx};

use super::{Request, HandlerMatch};

#[pyclass]
#[derive(Clone)]
pub struct RequestCtx {
    pub(crate) teo_inner: TeoRequestCtx,
}

/// HTTP request.
#[pymethods]
impl RequestCtx {

    pub fn request(&self) -> Request {
        Request {
            teo_request: self.teo_inner.request().clone()
        }
    }

    pub fn body(&self, py: Python<'_>) -> PyResult<PyObject> {
        teo_value_to_py_any(py, self.teo_inner.body())
    }

    pub fn teo(&self, py: Python<'_>) -> PyResult<PyObject> {
        Ok(py_ctx_object_from_teo_transaction_ctx(py, self.teo_inner.transaction_ctx(), "")?)
    }

    pub fn handler_match(&self) -> HandlerMatch {
        let static_self: &'static RequestCtx = unsafe { &*(self as * const RequestCtx) };
        HandlerMatch {
            teo_inner: static_self.teo_inner.handler_match()
        }
    }

    pub fn path_arguments(&self, py: Python<'_>) -> PyResult<PyObject> {
        self.handler_match().captures(py)
    }
}
