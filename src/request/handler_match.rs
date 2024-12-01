use pyo3::{pyclass, pymethods, types::{PyDict, PyDictMethods}, PyObject, PyResult, Python};
use teo::prelude::handler::r#match::HandlerMatch as TeoHandlerMatch;

#[pyclass]
pub struct HandlerMatch {
    pub(crate) teo_inner: TeoHandlerMatch,
}

/// Handler match.
#[pymethods]
impl HandlerMatch {

    #[getter]
    pub fn path(&self) -> Vec<String> {
        self.teo_inner.path().clone()
    }

    #[getter]
    pub fn handler_name(&self) -> &str {
        self.teo_inner.handler_name()
    }

    #[getter]
    pub fn captures(&self, py: Python<'_>) -> PyResult<PyObject> {
        let captures_map = self.teo_inner.captures();
        let py_dict = PyDict::new(py);
        for (k, value) in captures_map.iter() {
            py_dict.set_item(k, value)?;
        }
        Ok(py_dict.unbind().into_any())
    }
}
