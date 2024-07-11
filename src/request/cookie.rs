use pyo3::pyclass;
use teo::prelude::request::Cookie as ActixCookie;

#[pyclass]
#[derive(Clone)]
pub struct Cookie {
    pub(crate) actix_cookie: ActixCookie<'static>,
}

impl Cookie {

}