use pyo3::{pyclass, pymethods, Bound, PyAny, PyObject, PyResult, Python};
use teo::prelude::{request::local_values::LocalValues as TeoLocalValues, Value};

use crate::object::value::{py_any_to_teo_value, teo_value_to_py_any_without_model_objects};

#[pyclass]
#[derive(Clone)]
pub struct LocalValues {
    pub(crate) teo_local_values: TeoLocalValues,
}

impl LocalValues {
    pub(crate) fn new(teo_local_values: TeoLocalValues) -> Self {
        Self {
            teo_local_values
        }
    }
}

#[pymethods]
impl LocalValues {

    pub fn insert(&self, key: String, value: Bound<PyAny>, py: Python<'_>) -> PyResult<()> {
        let value = py_any_to_teo_value(py, &value)?;
        self.teo_local_values.insert(key, value);
        Ok(())
    }

    pub fn get(&self, key: String, py: Python<'_>) -> PyResult<PyObject> {
        let value: &Value = self.teo_local_values.get(&key)?;
        Ok(teo_value_to_py_any_without_model_objects(py, value)?)
    }

    pub fn contains(&self, key: String) -> bool {
        self.teo_local_values.contains(&key)
    }

    pub fn remove(&self, key: String) {
        self.teo_local_values.remove(key.as_str());
    }

    pub fn clear(&self) {
        self.teo_local_values.clear();
    }
}