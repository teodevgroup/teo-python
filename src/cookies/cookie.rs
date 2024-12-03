use pyo3::PyResult;
use pyo3::{pyclass, pymethods};
use teo::prelude::cookies::SameSite;
use time::Duration;
use teo::prelude::cookies::Cookie as OriginalCookie;
use super::expiration::Expiration;

#[pyclass]
#[derive(Clone)]
pub struct Cookie {
    pub(crate) original: OriginalCookie,
}

#[pymethods]
impl Cookie {

    #[new]
    pub fn new(name: &str, value: &str) -> Self {
        Cookie { original: OriginalCookie::new(name.to_owned(), value.to_owned()) }
    }

    #[getter]
    pub fn name(&self) -> String {
        self.original.name()
    }

    #[setter]
    pub fn set_name(&self, name: &str) {
        self.original.set_name(name);
    }

    #[getter]
    pub fn value(&self) -> String {
        self.original.value()
    }

    #[setter]
    pub fn set_value(&self, value: &str) {
        self.original.set_value(value);
    }

    #[getter]
    pub fn value_trimmed(&self) -> String {
        self.original.value_trimmed()
    }

    #[getter]
    pub fn http_only(&self) -> Option<bool> {
        self.original.http_only()
    }

    #[setter]
    pub fn set_http_only(&self, http_only: Option<bool>) {
        self.original.set_http_only(http_only)
    }

    #[getter]
    pub fn secure(&self) -> Option<bool> {
        self.original.secure()
    }

    #[setter]
    pub fn set_secure(&self, secure: Option<bool>) {
        self.original.set_secure(secure)
    }

    #[getter]
    pub fn same_site(&self) -> Option<&'static str> {
        self.original.same_site().map(|s| {
            match s {
                SameSite::Strict => "strict",
                SameSite::Lax => "lax",
                SameSite::None => "none",
            }
        })
    }

    #[setter]
    pub fn set_same_site(&self, same_site: Option<&str>) -> PyResult<()> {
        Ok(self.original.set_same_site(match same_site {
            None => None,
            Some(same_site) => match same_site {
                "strict" => Some(SameSite::Strict),
                "lax" => Some(SameSite::Lax),
                "none" => Some(SameSite::None),
                _ => Err(teo_result::Error::internal_server_error_message("invalid same site"))?
            }
        }))
    }

    #[getter]
    pub fn partitioned(&self) -> Option<bool> {
        self.original.partitioned()
    }

    #[setter]
    pub fn set_partitioned(&self, partitioned: Option<bool>) {
        self.original.set_partitioned(partitioned)
    }

    #[getter]
    pub fn max_age(&self) -> Option<f64> {
        self.original.max_age().map(|ma| ma.as_seconds_f64())
    }

    #[setter]
    pub fn set_max_age(&self, max_age: Option<f64>) {
        self.original.set_max_age(max_age.map(|d| Duration::seconds(d as i64)));
    }

    #[getter]
    pub fn path(&self) -> Option<String> {
        self.original.path()
    }

    #[setter]
    pub fn set_path(&self, path: Option<String>) {
        self.original.set_path(path)
    }

    #[getter]
    pub fn domain(&self) -> Option<String> {
        self.original.domain()
    }

    #[setter]
    pub fn set_domain(&self, domain: Option<String>) {
        self.original.set_domain(domain)
    }

    #[getter]
    pub fn expires(&self) -> Option<Expiration> {
        self.original.expires().map(|e| Expiration { original: e })
    }

    #[setter]
    pub fn set_expires(&self, expires: Option<Expiration>) {
        self.original.set_expires(expires.map(|e| e.original));
    }

    pub fn __str__(&self) -> String {
        format!("{}", self.original)
    }

    pub fn __repr__(&self) -> String {
        format!("{:?}", self.original)
    }

    pub fn make_removal(&self) {
        self.original.make_removal()
    }

    pub fn make_permanent(&self) {
        self.original.make_permanent()
    }

    pub fn encoded(&self) -> String {
        self.original.encoded()
    }
}