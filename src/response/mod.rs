use std::path::PathBuf;
use crate::{cookies::Cookies, headers::Headers, object::value::{py_any_to_teo_value, teo_value_to_py_any_without_model_objects}};
use pyo3::{pyclass, pymethods, types::PyNone, Bound, PyAny, PyObject, PyResult, Python};
use teo::prelude::response::Response as OriginalResponse;

#[pyclass]
#[derive(Clone)]
pub struct Response {
    pub(crate) original: OriginalResponse,
}

impl From<OriginalResponse> for Response {
    fn from(original: OriginalResponse) -> Self {
        Self {
            original
        }
    }
}

#[pymethods]
impl Response {

    #[staticmethod]
    pub fn empty() -> Self {
        Self {
            original: OriginalResponse::empty()
        }
    }

    #[staticmethod]
    pub fn string(content: String, content_type: String) -> PyResult<Self> {
        Ok(Self {
            original: OriginalResponse::string(content, &content_type.as_str())?
        })
    }

    #[staticmethod]
    pub fn teon(py: Python<'_>, value: Bound<PyAny>) -> PyResult<Self> {
        let teo_value = py_any_to_teo_value(py, &value)?;
        let response = OriginalResponse::teon(teo_value);
        Ok(Self {
            original: response
        })
    }

    #[staticmethod]
    pub fn html(content: String) -> PyResult<Self> {
        Ok(Self::string(content, "text/html".to_owned())?)
    }

    #[staticmethod]
    pub fn data(py: Python<'_>, value: Bound<PyAny>) -> PyResult<Self> {
        let teo_value = py_any_to_teo_value(py, &value)?;
        let response = OriginalResponse::data(teo_value);
        Ok(Self {
            original: response
        })
    }
    
    #[staticmethod]
    pub fn data_meta(py: Python<'_>, data: Bound<PyAny>, meta: Bound<PyAny>) -> PyResult<Self> {
        let teo_data = py_any_to_teo_value(py, &data)?;
        let teo_meta = py_any_to_teo_value(py, &meta)?;
        let response = OriginalResponse::data_meta(teo_data, teo_meta);
        Ok(Self {
            original: response
        })
    }
    
    #[staticmethod]
    pub fn file(path: String) -> Self {
        let path_buf = PathBuf::from(path);
        Self {
            original: OriginalResponse::file(path_buf)
        }
    }

    #[staticmethod]
    pub fn send_file(base: &str, path: &str) -> PyResult<Self> {
        Ok(Self {
            original: OriginalResponse::send_file(base, path)?
        })
    }

    #[staticmethod]
    pub fn redirect(path: String) -> PyResult<Self> {
        Ok(Self {
            original: OriginalResponse::redirect(path)?
        })
    }

    #[setter]
    pub fn set_code(&self, code: u16) {
        self.original.set_code(code)
    }

    #[getter]
    pub fn code(&self) -> u16 {
        self.original.code()
    }

    #[getter]
    pub fn headers(&self) -> Headers {
        Headers::from(self.original.headers())
    }

    #[setter]
    pub fn set_headers(&self, headers: &Headers) {
        self.original.set_headers(headers.original().clone());
    }

    #[getter]
    pub fn is_file(&self) -> bool {
        self.original.body().is_file()
    }

    #[getter]
    pub fn is_text(&self) -> bool {
        self.original.body().is_text()
    }

    #[getter]
    pub fn is_empty(&self) -> bool {
        self.original.body().is_empty()
    }

    #[getter]
    pub fn is_teon(&self) -> bool {
        self.original.body().is_teon()
    }

    #[getter]
    pub fn get_text(&self) -> Option<String> {
        self.original.body().as_text().cloned()
    }

    #[getter]
    pub fn get_teon(&self, py: Python<'_>) -> PyResult<PyObject> {
        Ok(match self.original.body().as_teon() {
            None => PyNone::get(py).as_unbound().clone_ref(py).into_any(),
            Some(value) => teo_value_to_py_any_without_model_objects(py, value)?
        })
    }

    #[getter]
    pub fn get_file(&self) -> Option<String> {
        match self.original.body().as_file() {
            None => None,
            Some(path_buf) => Some(path_buf.to_str().unwrap().to_string()),
        }
    }

    #[getter]
    pub fn cookies(&self) -> Cookies {
        Cookies::from(self.original.cookies())
    }

    #[setter]
    pub fn set_cookies(&self, cookies: &Cookies) {
        self.original.set_cookies(cookies.original().clone());
    }
}
