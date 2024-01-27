use pyo3::{pyclass, pymethods, Python, PyResult, PyObject, types::PyDict, IntoPy};
use teo::prelude::handler::r#match::HandlerMatch as TeoHandlerMatch;

#[pyclass]
pub struct HandlerMatch {
    pub(crate) teo_inner: &'static TeoHandlerMatch,
}

/// Handler match.
#[pymethods]
impl HandlerMatch {

    pub fn path(&self) -> Vec<&str> {
        self.teo_inner.path()
    }

    pub fn handler_name(&self) -> &str {
        self.teo_inner.handler_name()
    }

    pub fn captures(&self, py: Python<'_>) -> PyResult<PyObject> {
        let captures_map = self.teo_inner.captures();
        let py_dict = PyDict::new(py);
        for (k, value) in captures_map.iter() {
            py_dict.set_item(k, value)?;
        }
        Ok(py_dict.into_py(py))
    }
}
