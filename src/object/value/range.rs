use pyo3::{pyclass, pymethods, PyResult, PyAny, Python};
use teo::prelude::Range as TeoRange;

use super::teo_value_to_py_any;

#[pyclass]
pub struct Range {
    pub(crate) value: TeoRange
}

#[pymethods]
impl Range {

    pub fn upperbond(&self, py: Python<'_>) -> PyResult<PyAny> {
        let value = self.value.end.as_ref();
        let any = teo_value_to_py_any(py, value)?;
        Ok(any.coerce_to_number()?)
    }

    pub fn lowerbond(&self, py: Python<'_>) -> PyResult<PyAny> {
        let value = self.value.start.as_ref();
        let any = teo_value_to_py_any(py, value)?;
        Ok(any.coerce_to_number()?)
    }

    pub fn is_closed(&self) -> bool {
        self.value.closed
    }

    pub fn is_open(&self) -> bool {
        !self.value.closed
    }
}