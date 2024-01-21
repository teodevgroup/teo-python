use pyo3::{pyclass, pymethods};
use teo::prelude::request::header::readonly::HeaderMap;

#[pyclass]
pub struct ReadOnlyHeaderMap {
    pub(super) inner: HeaderMap
}

#[pymethods]
impl ReadOnlyHeaderMap {

    pub fn keys(&self) -> Vec<&str> {
        self.inner.keys()
    }

    pub fn len(&self) -> i64 {
        self.inner.len() as i64
    }

    pub fn contains_key(&self, key: String) -> bool {
        self.inner.contains_key(key)
    }

    pub fn get(&self, key: String) -> Option<&str> {
        self.inner.get(key)
    }
}