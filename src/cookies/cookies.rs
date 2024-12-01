use pyo3::PyResult;
use pyo3::{pyclass, pymethods};
use teo::prelude::cookies::SameSite;
use time::Duration;
use teo::prelude::cookies::Cookies as OriginalCookies;
use super::expiration::Expiration;

#[pyclass]
#[derive(Clone)]
pub struct Cookies {
    pub(crate) original: OriginalCookies,
}

#[pymethods]
impl Cookies {
    
}