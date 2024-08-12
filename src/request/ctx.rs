use pyo3::{pyclass, pymethods, PyObject, PyResult, Python};
use teo::prelude::request::Ctx as TeoRequestCtx;
use crate::{dynamic::py_class_lookup_map::PYClassLookupMap, object::value::teo_value_to_py_any_without_model_objects};

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
        teo_value_to_py_any_without_model_objects(py, self.teo_inner.body())
    }

    pub fn teo(&self, py: Python<'_>) -> PyResult<PyObject> {
        let map = PYClassLookupMap::from_app_data(self.teo_inner.namespace().app_data());
        map.teo_transaction_ctx_to_py_ctx_object(py, self.teo_inner.transaction_ctx(), "")
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
