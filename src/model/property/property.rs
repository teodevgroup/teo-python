use pyo3::{PyResult, Python, pyclass, pymethods, IntoPy, PyObject};
use teo::prelude::model::Property as TeoProperty;

use crate::object::value::{teo_value_to_py_any, py_any_to_teo_value};

#[pyclass]
pub struct Property {
    pub(crate) teo_property: &'static mut TeoProperty,
}

#[pymethods]
impl Property {

    pub fn set_data(&mut self, py: Python<'_>, key: String, value: PyObject) -> PyResult<()> {
        self.teo_property.data.insert(key, py_any_to_teo_value(py, value.as_ref(py))?);
        Ok(())
    }

    pub fn data(&mut self, py: Python<'_>, key: String) -> PyResult<PyObject> {
        Ok(match self.teo_property.data.get(key.as_str()) {
            Some(object) => teo_value_to_py_any(py, object)?,
            None => ().into_py(py),
        })
    }
}