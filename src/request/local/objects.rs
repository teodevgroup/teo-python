use pyo3::{pyclass, pymethods, Bound, IntoPy, PyAny, PyObject, PyResult, Python};
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

    pub fn insert(&self, key: String, value: Bound<PyAny>, py: Python<'_>) -> PyResult<()> {
        let value = value.into_py(py);
        self.teo_local_objects.insert(key, value);
        Ok(())
    }

    pub fn get(&self, key: String, py: Python<'_>) -> PyResult<PyObject> {
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

    pub fn contains(&self, key: String) -> bool {
        self.teo_local_objects.contains(&key)
    }

    pub fn remove(&self, key: String) {
        self.teo_local_objects.remove(key.as_str());
    }

    pub fn clear(&self) {
        self.teo_local_objects.clear();
    }
}