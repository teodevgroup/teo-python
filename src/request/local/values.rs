use pyo3::{pyclass, pymethods, Bound, PyAny, PyObject, PyResult, Python};
use teo::prelude::{request::local_values::LocalValues as OriginalLocalValues, Value};

use crate::object::value::{py_any_to_teo_value, teo_value_to_py_any_without_model_objects};

#[pyclass]
#[derive(Clone)]
pub struct LocalValues {
    pub(crate) original: OriginalLocalValues,
}

impl LocalValues {
    pub(crate) fn new(original: OriginalLocalValues) -> Self {
        Self {
            original
        }
    }
}

#[pymethods]
impl LocalValues {

    pub fn __setitem__(&self, key: String, value: Bound<PyAny>, py: Python<'_>) -> PyResult<()> {
        let value = py_any_to_teo_value(py, &value)?;
        self.original.insert(key, value);
        Ok(())
    }

    pub fn __getitem__(&self, key: String, py: Python<'_>) -> PyResult<PyObject> {
        let value: &Value = self.original.get(&key)?;
        Ok(teo_value_to_py_any_without_model_objects(py, value)?)
    }

    pub fn __hasitem__(&self, key: String) -> bool {
        self.original.contains(&key)
    }

    pub fn __delitem__(&self, key: String) {
        self.original.remove(key.as_str());
    }

    pub fn __len__(&self) -> usize {
        self.original.len()
    }

    pub fn clear(&self) {
        self.original.clear();
    }
}