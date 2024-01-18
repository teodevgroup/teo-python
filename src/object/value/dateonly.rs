use chrono::NaiveDate;
use pyo3::{pyclass, pymethods, Python, PyResult, exceptions::PyValueError};

#[pyclass]
pub struct DateOnly {
    pub(crate) value: NaiveDate
}

#[pymethods]
impl DateOnly {

    pub fn to_string(&self) -> String {
        self.value.format("%Y-%m-%d").to_string()
    }

    pub fn from_string<'p>(py: Python<'p>, string: String) -> PyResult<DateOnly> {
        match NaiveDate::parse_from_str(string.as_str(), "%Y-%m-%d") {
            Ok(value) => Ok(Self { value }),
            Err(e) => {
                Err(PyValueError::new_err("argument is not valid date only string"))
            }
        }
    }
}