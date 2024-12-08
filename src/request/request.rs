use pyo3::{pyclass, pymethods, Bound, PyAny, PyObject, PyResult, Python};
use teo::prelude::request::{Version, Method, Request as OriginalRequest};
use teo_result::Error;
use crate::{cookies::cookies::Cookies, dynamic::{DynamicClasses, QueryDynamicClasses}, headers::Headers, object::value::{py_any_to_teo_value, teo_value_to_py_any_without_model_objects}};
use super::{local::{objects::LocalObjects, values::LocalValues}, HandlerMatch};

#[pyclass]
#[derive(Clone)]
pub struct Request {
    pub(crate) original: OriginalRequest,
}

impl From<OriginalRequest> for Request {
    fn from(original: OriginalRequest) -> Self {
        Self { original }
    }
}

/// HTTP request.
#[pymethods]
impl Request {

    #[getter]
    pub fn version(&self) -> String {
        format!("{:?}", self.original.version())
    }

    #[setter]
    pub fn set_version(&self, value: &str) -> PyResult<()> {
        match value {
            "HTTP/0.9" => { self.original.set_version(Version::HTTP_09); },
            "HTTP/1.0" => { self.original.set_version(Version::HTTP_10); },
            "HTTP/1.1" => { self.original.set_version(Version::HTTP_11); },
            "HTTP/2.0" => { self.original.set_version(Version::HTTP_2); },
            "HTTP/3.0" => { self.original.set_version(Version::HTTP_3); },
            _ => { Err(Error::new(format!("Invalid version: {}", value)))? },
        }
        Ok(())
    }

    #[getter]
    pub fn method(&self) -> &str {
        self.original.method().as_str()
    }

    #[setter]
    pub fn set_method(&self, value: &str) -> PyResult<()> {
        let method = Method::from_bytes(value.as_bytes()).unwrap();
        self.original.set_method(method);
        Ok(())
    }

    #[getter]
    pub fn uri(&self) -> String {
        self.original.uri_string()
    }

    #[setter]
    pub fn set_uri(&self, value: &str) -> PyResult<()> {
        self.original.set_uri_string(value)?;
        Ok(())
    }

    #[getter]
    pub fn scheme(&self) -> Option<&str> {
        self.original.scheme().map(|s| s.as_str())
    }

    #[getter]
    pub fn host(&self) -> Option<&str> {
        self.original.host()
    }

    #[getter]
    pub fn path(&self) -> &str {
        self.original.path()
    }

    #[getter]
    pub fn query(&self) -> Option<&str> {
        self.original.query()
    }
    
    #[getter]
    pub fn content_type(&self) -> PyResult<Option<String>> {
        Ok(self.original.content_type()?)
    }

    #[getter]
    pub fn headers(&self) -> Headers {
        Headers::from(self.original.headers())
    }

    #[setter]
    pub fn set_headers(&self, headers: &Headers) {
        self.original.set_headers(headers.original().clone());
    }

    #[getter]
    pub fn cookies(&self) -> PyResult<Cookies> {
        Ok(Cookies::from(self.original.cookies()?.clone()))
    }

    #[setter]
    pub fn set_cookies(&self, cookies: &Cookies) {
        self.original.set_cookies(cookies.original().clone());
    }

    #[getter]
    pub fn handler_match(&self) -> PyResult<HandlerMatch> {
        Ok(HandlerMatch { 
            teo_inner: self.original.handler_match()?.clone()
        })
    }

    #[getter]
    pub fn captures(&self, py: Python<'_>) -> PyResult<PyObject> {
        self.handler_match()?.captures(py)
    }

    #[getter]
    pub fn body_object(&self, py: Python<'_>) -> PyResult<PyObject> {
        teo_value_to_py_any_without_model_objects(py, self.original.body_value()?)
    }

    #[setter]
    pub fn set_body_object(&self, py: Python<'_>, value: &Bound<PyAny>) -> PyResult<()> {
        let value = py_any_to_teo_value(py, value)?;
        self.original.set_body_value(value);
        Ok(())
    }

    #[getter]
    pub fn teo(&self, py: Python<'_>) -> PyResult<PyObject> {
        let map = DynamicClasses::retrieve(self.original.transaction_ctx().namespace().app_data())?;
        map.teo_transaction_ctx_to_py_ctx_object(py, self.original.transaction_ctx(), "")
    }

    #[getter]
    pub fn local_values(&self) -> LocalValues {
        LocalValues::new(self.original.local_values().clone())
    }

    #[getter]
    pub fn local_objects(&self) -> LocalObjects {
        LocalObjects::new(self.original.local_objects().clone())
    }

    // TODO: take incoming as stream? temp file? string?
}
