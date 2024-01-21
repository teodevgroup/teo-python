
use pyo3::{pyclass, pymethods};
use teo::prelude::Request as TeoRequest;

use super::header_map::ReadOnlyHeaderMap;


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

    pub fn headers(&self) -> ReadOnlyHeaderMap {
        ReadOnlyHeaderMap {
            inner: self.teo_request.headers().clone()
        }
    }
}
