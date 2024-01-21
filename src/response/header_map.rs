use pyo3::{pyclass, pymethods};
use teo::prelude::response::header::readwrite::HeaderMap;

#[pyclass]
pub struct ReadWriteHeaderMap {
    pub(super) inner: HeaderMap
}

#[pymethods]
impl ReadWriteHeaderMap {

    pub fn keys(&self) -> Vec<String> {
        self.inner.keys()
    }

    pub fn len(&self) -> i64 {
        self.inner.len() as i64
    }

    pub fn contains_key(&self, key: String) -> bool {
        self.inner.contains_key(key)
    }

    pub fn get(&self, key: String) -> Option<String> {
        self.inner.get(key)
    }

    pub fn set(&self, key: String, value: String) {
        self.inner.set(key, value)
    }
}