use pyo3::{pyclass, pymethods};
use std::str::FromStr;
use hyper::{header::{HeaderName, HeaderValue}, HeaderMap, Method};
use http_body_util::Full;
use bytes::Bytes;


#[pyclass]
pub struct TestRequest {
    method: Method,
    uri: String,
    headers: HeaderMap,
    body: Bytes,
}

#[pymethods]
impl TestRequest {
    
}