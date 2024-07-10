use pyo3::{pyclass, pymethods, types::PyAnyMethods, IntoPy, PyAny, PyObject, PyResult, Python};
use teo::prelude::model;

use crate::object::value::{teo_value_to_py_any, py_any_to_teo_value};

#[pyclass]
pub struct Property {
    pub(crate) teo_property: model::property::Builder,
}

#[pymethods]
impl Property {

    pub fn set_data(&mut self, py: Python<'_>, key: String, value: PyObject) -> PyResult<()> {
        self.teo_property.data().insert(key, py_any_to_teo_value(py, value.bind(py).extract::<&PyAny>()?)?);
        Ok(())
    }

    pub fn data(&mut self, py: Python<'_>, key: String) -> PyResult<PyObject> {
        Ok(match self.teo_property.data().get(key.as_str()) {
            Some(object) => teo_value_to_py_any(py, object)?,
            None => ().into_py(py),
        })
    }
}