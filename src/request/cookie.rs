use pyo3::PyResult;
use pyo3::{pyclass, pymethods};
use teo::prelude::request::Cookie as ActixCookie;
use teo::prelude::request::SameSite;
use time::Duration;
use super::Expiration;

#[pyclass]
#[derive(Clone)]
pub struct Cookie {
    pub(crate) actix_cookie: ActixCookie<'static>,
}

#[pymethods]
impl Cookie {

    pub fn name(&self) -> &str {
        self.actix_cookie.name()
    }

    pub fn value(&self) -> &str {
        self.actix_cookie.value()
    }

    pub fn path(&self) -> Option<&str> {
        self.actix_cookie.path()
    }

    pub fn domain(&self) -> Option<&str> {
        self.actix_cookie.domain()
    }

    pub fn max_age(&self) -> Option<f64> {
        self.actix_cookie.max_age().map(|d| d.as_seconds_f64())
    }

    pub fn expires(&self) -> Option<Expiration> {
        self.actix_cookie.expires().map(|e| Expiration { inner: e })
    }

    pub fn secure(&self) -> Option<bool> {
        self.actix_cookie.secure()
    }

    pub fn http_only(&self) -> Option<bool> {
        self.actix_cookie.http_only()
    }

    pub fn same_site(&self) -> Option<&'static str> {
        self.actix_cookie.same_site().map(|s| {
            match s {
                SameSite::Strict => "strict",
                SameSite::Lax => "lax",
                SameSite::None => "none",
            }
        })
    }

    pub fn set_name(&mut self, name: String) {
        self.actix_cookie.set_name(name);
    }

    pub fn set_value(&mut self, value: String) {
        self.actix_cookie.set_value(value);
    }

    pub fn set_path(&mut self, path: String) {
        self.actix_cookie.set_path(path);
    }

    pub fn set_domain(&mut self, domain: String) {
        self.actix_cookie.set_domain(domain);
    }

    pub fn set_max_age(&mut self, max_age: Option<f64>) {
        self.actix_cookie.set_max_age(max_age.map(|d| Duration::seconds(d as i64)));
    }

    pub fn set_expires(&mut self, expires: Expiration) {
        self.actix_cookie.set_expires(expires.inner);
    }

    pub fn set_secure(&mut self, secure: Option<bool>) {
        self.actix_cookie.set_secure(secure);
    }

    pub fn set_http_only(&mut self, http_only: Option<bool>) {
        self.actix_cookie.set_http_only(http_only);
    }

    pub fn set_same_site(&mut self, same_site: &str) -> PyResult<()> {
        Ok(self.actix_cookie.set_same_site(match same_site {
            "strict" => SameSite::Strict,
            "lax" => SameSite::Lax,
            "none" => SameSite::None,
            _ => Err(teo::result::Error::internal_server_error_message("invalid same site"))?
        }))
    }

    pub fn to_string(&self) -> String {
        self.actix_cookie.to_string()
    }

    #[staticmethod]
    pub fn from_string(s: String) -> PyResult<Cookie> {
        let result = ActixCookie::parse(s);
        match result {
            Ok(result) => Ok(Cookie { actix_cookie: result }),
            Err(_) => Err(teo::result::Error::internal_server_error_message("invalid cookie string"))?
        }
    }

    pub fn make_removal(&mut self) {
        self.actix_cookie.make_removal()
    }

    pub fn make_permanent(&mut self) {
        self.actix_cookie.make_permanent()
    }
}