pub mod header_map;

use std::path::PathBuf;
use crate::{object::value::{py_any_to_teo_value, teo_value_to_py_any_without_model_objects}, request::Cookie};
use self::header_map::ReadWriteHeaderMap;
use pyo3::{pyclass, pymethods, Bound, IntoPy, PyAny, PyObject, PyResult, Python};
use teo::prelude::response::Response as TeoResponse;

#[pyclass]
#[derive(Clone)]
pub struct Response {
    pub(crate) teo_response: TeoResponse,
}

#[pymethods]
impl Response {

    #[staticmethod]
    pub fn empty() -> Self {
        Self {
            teo_response: TeoResponse::empty()
        }
    }

    #[staticmethod]
    pub fn string(content: String, content_type: String) -> Self {
        Self {
            teo_response: TeoResponse::string(content, &content_type.as_str())
        }
    }

    #[staticmethod]
    pub fn teon(py: Python<'_>, value: Bound<PyAny>) -> PyResult<Self> {
        let teo_value = py_any_to_teo_value(py, &value)?;
        let response = TeoResponse::teon(teo_value);
        Ok(Self {
            teo_response: response
        })
    }

    #[staticmethod]
    pub fn html(content: String) -> Self {
        Self::string(content, "text/html".to_owned())
    }

    #[staticmethod]
    pub fn data(py: Python<'_>, value: Bound<PyAny>) -> PyResult<Self> {
        let teo_value = py_any_to_teo_value(py, &value)?;
        let response = TeoResponse::data(teo_value);
        Ok(Self {
            teo_response: response
        })
    }
    
    #[staticmethod]
    pub fn data_meta(py: Python<'_>, data: Bound<PyAny>, meta: Bound<PyAny>) -> PyResult<Self> {
        let teo_data = py_any_to_teo_value(py, &data)?;
        let teo_meta = py_any_to_teo_value(py, &meta)?;
        let response = TeoResponse::data_meta(teo_data, teo_meta);
        Ok(Self {
            teo_response: response
        })
    }
    
    #[staticmethod]
    pub fn file(path: String) -> Self {
        let path_buf = PathBuf::from(path);
        Self {
            teo_response: TeoResponse::file(path_buf)
        }
    }

    #[staticmethod]
    pub fn redirect(path: String) -> Self {
        Self {
            teo_response: TeoResponse::redirect(path)
        }
    }

    pub fn set_code(&self, code: u16) {
        self.teo_response.set_code(code)
    }

    pub fn code(&self) -> u16 {
        self.teo_response.code()
    }

    pub fn headers(&self) -> ReadWriteHeaderMap {
        ReadWriteHeaderMap {
            inner: self.teo_response.headers()
        }
    }

    pub fn is_file(&self) -> bool {
        self.teo_response.body().is_file()
    }

    pub fn is_text(&self) -> bool {
        self.teo_response.body().is_text()
    }

    pub fn is_empty(&self) -> bool {
        self.teo_response.body().is_empty()
    }

    pub fn is_teon(&self) -> bool {
        self.teo_response.body().is_teon()
    }

    pub fn get_text(&self) -> Option<String> {
        self.teo_response.body().as_text().cloned()
    }

    pub fn get_teon(&self, py: Python<'_>) -> PyResult<PyObject> {
        Ok(match self.teo_response.body().as_teon() {
            None => ().into_py(py),
            Some(value) => teo_value_to_py_any_without_model_objects(py, value)?
        })
    }

    pub fn get_file(&self) -> Option<String> {
        match self.teo_response.body().as_file() {
            None => None,
            Some(path_buf) => Some(path_buf.to_str().unwrap().to_string()),
        }
    }

    pub fn add_cookie(&self, cookie: Cookie) {
        self.teo_response.add_cookie(cookie.teo_cookie)
    }

    pub fn cookies(&self) -> Vec<Cookie> {
        self.teo_response.cookies().into_iter().map(|c| Cookie { teo_cookie: c }).collect()
    }
}
