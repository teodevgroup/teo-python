use pyo3::{pyclass, pymethods, Bound, PyAny, PyObject, PyResult, Python};
use teo::prelude::request::local_objects::LocalObjects as TeoLocalObjects;

#[pyclass]
#[derive(Clone)]
pub struct LocalObjects {
    pub(crate) teo_local_objects: TeoLocalObjects,
}

impl LocalObjects {
    pub(crate) fn new(teo_local_objects: TeoLocalObjects) -> Self {
        Self {
            teo_local_objects
        }
    }
}

#[pymethods]
impl LocalObjects {

    pub fn __setitem__(&self, key: String, value: Bound<PyAny>) -> PyResult<()> {
        let value = value.unbind();
        self.teo_local_objects.insert(key, value);
        Ok(())
    }

    pub fn __getitem__(&self, key: String, py: Python<'_>) -> PyResult<PyObject> {
        let value: Option<&PyObject> = self.teo_local_objects.get(&key);
        match value {
            Some(value) => {
                Ok(value.clone_ref(py))
            },
            None => {
                Ok(py.None())
            }
        }
    }

    pub fn __hasitem__(&self, key: String) -> bool {
        self.teo_local_objects.contains(&key)
    }

    pub fn __delitem__(&self, key: String) {
        self.teo_local_objects.remove(key.as_str());
    }

    pub fn clear(&self) {
        self.teo_local_objects.clear();
    }
}