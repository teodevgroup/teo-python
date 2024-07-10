
use pyo3::{PyObject, pyclass, pymethods, types::{PyAnyMethods, PyDict}, Bound, IntoPy, PyResult, Python};
use teo::prelude::Request as TeoRequest;


#[pyclass]
pub struct Request {
    pub(crate) teo_request: TeoRequest,
}

/// HTTP request.
#[pymethods]
impl Request {

    pub fn method(&self) -> &str {
        self.teo_request.method()
    }

    pub fn path(&self) -> &str {
        self.teo_request.path()
    }

    pub fn query_string(&self) -> &str {
        self.teo_request.query_string()
    }
    
    pub fn content_type(&self) -> &str {
        self.teo_request.content_type()
    }

    pub fn header(&self, name: &str) -> Option<&str> {
        self.teo_request.headers().get(name).map(|hv| hv.to_str().unwrap())
    }

    pub fn headers(&self, py: Python<'_>) -> PyResult<PyObject> {
        let dict = PyDict::new_bound(py);
        for (key, value) in self.teo_request.headers().iter() {
            dict.set_item(key.as_str(), value.to_str().unwrap())?;
        }
        Ok(dict.into_py(py))
    }
}
