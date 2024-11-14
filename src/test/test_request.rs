use pyo3::{pyclass, pymethods, types::{PyAnyMethods, PyDict, PyDictMethods}, Bound, PyAny, PyResult};
use teo_result::Error;
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

    #[new]
    #[pyo3(signature = (**kwds))]
    pub fn new(kwds: Option<&Bound<PyDict>>) -> PyResult<Self> {
        let Some(kwds) = kwds else {
            return Err(Error::new("requires argument"))?;
        };
        // HTTP Method
        let method_str: Option<String> = if let Some(value) = kwds.get_item("method")? {
            Some(value.extract()?)
        } else {
            None
        };
        let method = match method_str {
            Some(method) => match Method::from_str(&method) {
                Ok(method) => method,
                Err(_) => Err(teo_result::Error::internal_server_error_message("cannot parse HTTP method"))?,
            },
            None => Method::GET,
        };
        // URI
        let Some(uri) = kwds.get_item("uri")? else {
            return Err(Error::new("missing required argument: uri"))?;
        };
        let uri: String = uri.extract()?;
        let mut headers: HeaderMap = HeaderMap::new();
        let headers_any = kwds.get_item("headers")?;
        if let Some(headers_any) = headers_any {
            let headers_dict: Bound<PyDict> = headers_any.extract()?;
            for (k, v) in headers_dict.iter() {
                let k: &str = k.extract()?;
                let v: &str = v.extract()?;
                headers.insert(match HeaderName::try_from(k) {
                    Ok(value) => value,
                    Err(_) => return Err(teo_result::Error::internal_server_error_message("cannot parse header name").into()),
                }, match HeaderValue::from_str(v) {
                    Ok(value) => value,
                    Err(_) => return Err(teo_result::Error::internal_server_error_message("cannot parse header value").into()),
                });
            }
        }
        let body = kwds.get_item("body")?;
        let body = if let Some(body) = body {
            Bytes::new()
        } else {
            Bytes::new()
        };
        Ok(Self {
            method,
            uri,
            headers,
            body,
        })
    }
}