use pyo3::{pyclass, pymethods, Bound, PyAny, PyObject, PyResult, Python};
use teo::prelude::Request as TeoRequest;
use crate::{dynamic::py_class_lookup_map::PYClassLookupMap, headers::Headers, object::value::{py_any_to_teo_value, teo_value_to_py_any_without_model_objects}};

use super::{local::{objects::LocalObjects, values::LocalValues}, HandlerMatch};

#[pyclass]
#[derive(Clone)]
pub struct Request {
    pub(crate) original: TeoRequest,
}

impl Request {
    pub(crate) fn new(teo_request: TeoRequest) -> Self {
        Self {
            original: teo_request.clone(),
        }
    }
}

/// HTTP request.
#[pymethods]
impl Request {

    pub fn version(&self) -> String {
        format!("{:?}", self.original.version())
    }

    pub fn method(&self) -> &str {
        self.original.method().as_str()
    }

    pub fn uri(&self) -> String {
        self.original.uri_string()
    }

    pub fn scheme(&self) -> Option<&str> {
        self.original.scheme().map(|s| s.as_str())
    }

    pub fn host(&self) -> Option<&str> {
        self.original.host()
    }

    pub fn path(&self) -> &str {
        self.original.path()
    }

    pub fn query(&self) -> Option<&str> {
        self.original.query()
    }
    
    pub fn content_type(&self) -> PyResult<Option<String>> {
        Ok(self.original.content_type()?)
    }

    pub fn headers(&self) -> Headers {
        Headers {
            original: self.original.headers()
        }
    }

    pub fn cookie(&self, name: String) -> PyResult<Option<Cookie>> {
        Ok(self.original.cookies()?.get(&name).map(|c| Cookie { teo_cookie: c.clone() }))
    }

    pub fn cookies(&self) -> PyResult<Vec<Cookie>> {
        Ok(self.original.cookies()?.iter().map(|c| Cookie { teo_cookie: c.clone() }).collect())
    }

    pub fn handler_match(&self) -> PyResult<HandlerMatch> {
        Ok(HandlerMatch { 
            teo_inner: self.original.handler_match()?.clone()
        })
    }

    pub fn captures(&self, py: Python<'_>) -> PyResult<PyObject> {
        self.handler_match()?.captures(py)
    }

    pub fn body_object(&self, py: Python<'_>) -> PyResult<PyObject> {
        teo_value_to_py_any_without_model_objects(py, self.original.body_value()?)
    }

    pub fn set_body_object(&self, value: &Bound<PyAny>, py: Python<'_>) -> PyResult<()> {
        let value = py_any_to_teo_value(py, value)?;
        self.original.set_body_value(value);
        Ok(())
    }

    pub fn teo(&self, py: Python<'_>) -> PyResult<PyObject> {
        let map = PYClassLookupMap::from_app_data(self.original.transaction_ctx().namespace().app_data());
        map.teo_transaction_ctx_to_py_ctx_object(py, self.original.transaction_ctx(), "")
    }

    pub fn local_values(&self) -> LocalValues {
        LocalValues::new(self.original.local_values().clone())
    }

    pub fn local_objects(&self) -> LocalObjects {
        LocalObjects::new(self.original.local_objects().clone())
    }

    // TODO: take incoming as stream? temp file? string?
}
