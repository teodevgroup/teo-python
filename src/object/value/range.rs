use pyo3::{pyclass, pymethods, PyResult, Python, PyObject};
use teo::prelude::Range as OriginalRange;

use super::teo_value_to_py_any_without_model_objects;

#[pyclass]
#[derive(Clone)]
pub struct Range {
    pub(crate) original: OriginalRange
}

#[pymethods]
impl Range {

    #[getter]
    pub fn upperbond(&self, py: Python<'_>) -> PyResult<PyObject> {
        let value = self.original.end.as_ref();
        let any = teo_value_to_py_any_without_model_objects(py, value)?;
        Ok(any)
    }

    #[getter]
    pub fn lowerbond(&self, py: Python<'_>) -> PyResult<PyObject> {
        let value = self.original.start.as_ref();
        let any = teo_value_to_py_any_without_model_objects(py, value)?;
        Ok(any)
    }

    #[getter]
    pub fn is_closed(&self) -> bool {
        self.original.closed
    }

    #[getter]
    pub fn is_open(&self) -> bool {
        !self.original.closed
    }

    pub fn __str__(&self) -> String {
        format!("{}{}{}", self.original.start, if self.original.closed() { "..." } else { ".." }, self.original.end)
    }

    pub fn __repr__(&self) -> String {
        format!("teo.Range({})", self.__str__())
    }
}