use pyo3::{pyclass, pymethods, Bound, PyAny, PyObject, PyResult, Python};
use teo::prelude::Request as TeoRequest;
use crate::{dynamic::py_class_lookup_map::PYClassLookupMap, object::value::{py_any_to_teo_value, teo_value_to_py_any_without_model_objects}};

use super::{Cookie, HandlerMatch};

#[pyclass]
#[derive(Clone)]
pub struct Request {
    pub(crate) teo_request: TeoRequest,
}

/// HTTP request.
#[pymethods]
impl Request {

    pub fn version(&self) -> String {
        format!("{:?}", self.teo_request.version())
    }

    pub fn method(&self) -> &str {
        self.teo_request.method().as_str()
    }

    pub fn uri(&self) -> String {
        self.teo_request.uri_string()
    }

    pub fn scheme(&self) -> Option<&str> {
        self.teo_request.scheme().map(|s| s.as_str())
    }

    pub fn host(&self) -> Option<&str> {
        self.teo_request.host()
    }

    pub fn path(&self) -> &str {
        self.teo_request.path()
    }

    pub fn query(&self) -> Option<&str> {
        self.teo_request.query()
    }
    
    pub fn content_type(&self) -> PyResult<Option<&str>> {
        Ok(self.teo_request.content_type()?)
    }

    pub fn contains_header(&self, name: &str) -> bool {
        self.teo_request.headers().contains_key(name)
    }

    pub fn header_value(&self, name: &str) -> PyResult<Option<&str>> {
        let header_value = self.teo_request.headers().get(name);
        match header_value {
            None => Ok(None),
            Some(header_value) => {
                let header_value = header_value.to_str().map_err(|_| {
                    teo_result::Error::internal_server_error_message(format!("cannot read request header value: {}", name))
                })?;
                Ok(Some(header_value))
            }
        }
    }

    pub fn header_values(&self, name: &str) -> PyResult<Vec<&str>> {
        let header_values = self.teo_request.headers().get_all(name);
        let mut result = Vec::new();
        for header_value in header_values {
            let header_value = header_value.to_str().map_err(|_| {
                teo_result::Error::internal_server_error_message(format!("cannot read request header value: {}", name))
            })?;
            result.push(header_value);
        }
        Ok(result)
    }

    pub fn header_keys(&self) -> Vec<&str> {
        let header_map = self.teo_request.headers();
        let mut result = vec![];
        header_map.keys().for_each(|k| {
            result.push(k.as_str());
        });
        result
    }

    pub fn headers_length(&self) -> usize {
        self.teo_request.headers().len()
    }

    pub fn cookie(&self, name: String) -> PyResult<Option<Cookie>> {
        Ok(self.teo_request.cookies()?.get(&name).map(|c| Cookie { teo_cookie: c.clone() }))
    }

    pub fn cookies(&self) -> PyResult<Vec<Cookie>> {
        Ok(self.teo_request.cookies()?.iter().map(|c| Cookie { teo_cookie: c.clone() }).collect())
    }

    pub fn handler_match(&self) -> PyResult<HandlerMatch> {
        Ok(HandlerMatch { 
            teo_inner: self.teo_request.handler_match()?.clone()
        })
    }

    pub fn captures(&self, py: Python<'_>) -> PyResult<PyObject> {
        self.handler_match()?.captures(py)
    }

    pub fn body_object(&self, py: Python<'_>) -> PyResult<PyObject> {
        teo_value_to_py_any_without_model_objects(py, self.teo_request.body_value()?)
    }

    pub fn set_body_object(&self, value: &Bound<PyAny>, py: Python<'_>) -> PyResult<()> {
        let value = py_any_to_teo_value(py, value)?;
        self.teo_request.set_body_value(value);
        Ok(())
    }

    pub fn teo(&self, py: Python<'_>) -> PyResult<PyObject> {
        let map = PYClassLookupMap::from_app_data(self.teo_request.transaction_ctx().namespace().app_data());
        map.teo_transaction_ctx_to_py_ctx_object(py, self.teo_request.transaction_ctx(), "")
    }

    // TODO: local objects and local data

    // TODO: take incoming as stream? temp file? string?
}
