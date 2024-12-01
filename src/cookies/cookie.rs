use pyo3::PyResult;
use pyo3::{pyclass, pymethods};
use teo::prelude::request::Cookie as TeoCookie;
use teo::prelude::request::SameSite;
use time::Duration;
use super::expiration::Expiration;

#[pyclass]
#[derive(Clone)]
pub struct Cookie {
    pub(crate) teo_cookie: TeoCookie<'static>,
}

#[pymethods]
impl Cookie {

    #[new]
    pub fn new(name: &str, value: &str) -> Self {
        Cookie { teo_cookie: TeoCookie::new(name.to_owned(), value.to_owned()) }
    }

    pub fn name(&self) -> &str {
        self.teo_cookie.name()
    }

    pub fn value(&self) -> &str {
        self.teo_cookie.value()
    }

    pub fn expires(&self) -> Option<Expiration> {
        self.teo_cookie.expires().map(|e| Expiration { inner: e })
    }

    pub fn max_age(&self) -> Option<f64> {
        self.teo_cookie.max_age().map(|ma| ma.as_seconds_f64())
    }

    pub fn domain(&self) -> Option<&str> {
        self.teo_cookie.domain()
    }

    pub fn path(&self) -> Option<&str> {
        self.teo_cookie.path()
    }

    pub fn secure(&self) -> Option<bool> {
        self.teo_cookie.secure()
    }

    pub fn http_only(&self) -> Option<bool> {
        self.teo_cookie.http_only()
    }

    pub fn set_same_site(&mut self, same_site: String) -> PyResult<()> {
        Ok(self.teo_cookie.set_same_site(match same_site.as_str() {
            "strict" => SameSite::Strict,
            "lax" => SameSite::Lax,
            "none" => SameSite::None,
            _ => Err(teo_result::Error::internal_server_error_message("invalid same site"))?
        }))
    }

    pub fn same_site(&self) -> Option<&'static str> {
        self.teo_cookie.same_site().map(|s| {
            match s {
                SameSite::Strict => "strict",
                SameSite::Lax => "lax",
                SameSite::None => "none",
            }
        })
    }

    pub fn to_string(&self) -> String {
        self.teo_cookie.to_string()
    }

    #[pyo3(signature = (max_age=None))]
    pub fn set_max_age(&mut self, max_age: Option<f64>) {
        self.teo_cookie.set_max_age(max_age.map(|d| Duration::seconds(d as i64)));
    }

    pub fn set_expires(&mut self, expires: &Expiration) {
        self.teo_cookie.set_expires(expires.inner);
    }

    pub fn set_domain(&mut self, domain: String) {
        self.teo_cookie.set_domain(domain);
    }

    pub fn set_path(&mut self, path: String) {
        self.teo_cookie.set_path(path);
    }

    #[pyo3(signature = (secure=None))]
    pub fn set_secure(&mut self, secure: Option<bool>) {
        self.teo_cookie.set_secure(secure);
    }

    #[pyo3(signature = (http_only=None))]
    pub fn set_http_only(&mut self, http_only: Option<bool>) {
        self.teo_cookie.set_http_only(http_only);
    }

    pub fn set_name(&mut self, name: String) {
        self.teo_cookie.set_name(name);
    }

    pub fn set_value(&mut self, value: String) {
        self.teo_cookie.set_value(value);
    }

    pub fn make_removal(&mut self) {
        self.teo_cookie.make_removal()
    }

    pub fn make_permanent(&mut self) {
        self.teo_cookie.make_permanent()
    }

    #[staticmethod]
    pub fn from_string(s: String) -> PyResult<Cookie> {
        let result = TeoCookie::parse(s);
        match result {
            Ok(result) => Ok(Cookie { teo_cookie: result }),
            Err(_) => Err(teo::result::Error::internal_server_error_message("invalid cookie string"))?
        }
    }
}