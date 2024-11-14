use pyo3::{pyclass, pymethods, types::{PyAnyMethods, PyBytes, PyDict, PyDictMethods, PyString}, Bound, PyResult, Python};
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
    pub fn new(kwds: Option<&Bound<PyDict>>, py: Python<'_>) -> PyResult<Self> {
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
            if body.is_instance_of::<PyBytes>() {
                let py_bytes: Bound<PyBytes> = body.extract()?;
                let data: &[u8] = py_bytes.extract()?;
                Bytes::copy_from_slice(&data)
            } else if body.is_instance_of::<PyString>() {
                let py_string: Bound<PyString> = body.extract()?;
                let rust_str: &str = py_string.extract()?;
                let rust_u8_slice = rust_str.as_bytes();
                Bytes::copy_from_slice(rust_u8_slice)
            } else {
                let json = py.import_bound("json")?;
                let dumps = json.getattr("dumps")?;
                let result = dumps.call1((body,))?;
                let result_string: &str = result.extract()?;
                Bytes::copy_from_slice(result_string.as_bytes())
            }
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

    pub fn method(&self) -> &str {
        self.method.as_str()
    }

    pub fn set_method(&mut self, method: &str) -> PyResult<()> {
        match Method::from_str(method) {
            Ok(method) => {
                self.method = method;
                Ok(())
            },
            Err(_) => Err(teo_result::Error::internal_server_error_message("cannot parse HTTP method").into()),
        }
    }

    pub fn uri(&self) -> &str {
        self.uri.as_str()
    }

    pub fn set_uri(&mut self, uri: String) {
        self.uri = uri;
    }

    pub fn insert_header(&mut self, key: String, value: String) -> PyResult<()> {
        self.headers.insert(match HeaderName::try_from(key) {
            Ok(value) => value,
            Err(_) => return Err(teo_result::Error::internal_server_error_message("cannot parse header name").into()),
        }, match HeaderValue::from_str(value.as_str()) {
            Ok(value) => value,
            Err(_) => return Err(teo_result::Error::internal_server_error_message("cannot parse header value").into()),
        });
        Ok(())
    }

    pub fn append_header(&mut self, key: String, value: String) -> PyResult<()> {
        self.headers.append(match HeaderName::try_from(key) {
            Ok(value) => value,
            Err(_) => return Err(teo_result::Error::internal_server_error_message("cannot parse header name").into()),
        }, match HeaderValue::from_str(value.as_str()) {
            Ok(value) => value,
            Err(_) => return Err(teo_result::Error::internal_server_error_message("cannot parse header value").into()),
        });
        Ok(())
    }

    pub fn body<'py>(&self, py: Python<'py>) -> Bound<'py, PyBytes> {
        let vec_u8_data = Vec::<u8>::from(self.body.clone());
        PyBytes::new_bound(py, &vec_u8_data)
    }

    pub fn set_body(&mut self, body: Bound<PyBytes>) -> PyResult<()> {
        let data: &[u8] = body.extract()?;
        self.body = Bytes::copy_from_slice(&data);
        Ok(())
    }
}

impl TestRequest {
    pub(crate) fn to_hyper_request(&self) -> hyper::Request<Full<Bytes>> {
        let mut request = hyper::Request::builder()
            .method(self.method.clone())
            .uri(self.uri.clone());
        for (key, value) in self.headers.iter() {
            request = request.header(key.clone(), value.clone());
        }
        request.body(Full::new(self.body.clone())).unwrap()
    }
}