use pyo3::{pyclass, pymethods, PyResult, Python, PyObject};
use teo::prelude::Range as TeoRange;

use super::teo_value_to_py_any_without_model_objects;

#[pyclass]
#[derive(Clone)]
pub struct Range {
    pub(crate) value: TeoRange
}

#[pymethods]
impl Range {

    pub fn upperbond(&self, py: Python<'_>) -> PyResult<PyObject> {
        let value = self.value.end.as_ref();
        let any = teo_value_to_py_any_without_model_objects(py, value)?;
        Ok(any)
    }

    pub fn lowerbond(&self, py: Python<'_>) -> PyResult<PyObject> {
        let value = self.value.start.as_ref();
        let any = teo_value_to_py_any_without_model_objects(py, value)?;
        Ok(any)
    }

    pub fn is_closed(&self) -> bool {
        self.value.closed
    }

    pub fn is_open(&self) -> bool {
        !self.value.closed
    }
}