use pyo3::{PyResult, Python, pyclass, pymethods, IntoPy, PyObject};
use teo::prelude::model::Field as TeoField;

use crate::object::value::{py_any_to_teo_value, teo_value_to_py_any};

#[pyclass]
pub struct Field {
    pub(crate) teo_field: &'static mut TeoField,
}

#[pymethods]
impl Field {

    pub fn set_data(&mut self, py: Python<'_>, key: String, value: PyObject) -> PyResult<()> {
        self.teo_field.data.insert(key, py_any_to_teo_value(py, value.as_ref(py))?);
        Ok(())
    }

    pub fn data(&mut self, py: Python<'_>, key: String) -> PyResult<PyObject> {
        Ok(match self.teo_field.data.get(key.as_str()) {
            Some(object) => teo_value_to_py_any(py, object)?,
            None => ().into_py(py),
        })
    }
}