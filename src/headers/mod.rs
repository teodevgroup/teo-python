use pyo3::{pyclass, pymethods, types::{PyAnyMethods, PyList}, PyResult, Python};
use teo::prelude::headers::Headers as OriginalHeaders;

#[pyclass]
pub struct Headers {
    original: OriginalHeaders
}

impl From<OriginalHeaders> for Headers {
    fn from(original: OriginalHeaders) -> Self {
        Self { original }
    }
}

impl Headers {
    pub(crate) fn original(&self) -> &OriginalHeaders {
        &self.original
    }
}

#[pymethods]
impl Headers {

    pub fn keys(&self) -> Vec<String> {
        self.original.keys()
    }

    pub fn __len__(&self) -> usize {
        self.original.len()
    }

    pub fn __contains__(&self, key: &str) -> bool {
        self.original.contains_key(key)
    }

    pub fn __getitem__(&self, key: &str) -> PyResult<Option<String>> {
        Ok(self.original.get(key)?)
    }

    pub fn __setitem__(&self, key: String, value: String) -> PyResult<()> {
        Ok(self.original.insert(key, value)?)
    }

    pub fn append(&self, key: String, value: String) -> PyResult<()> {
        Ok(self.original.append(key, value)?)
    }

    pub fn __delitem__(&self, key: &str) {
        self.original.remove(key)
    }

    pub fn clear(&self) {
        self.original.clear()
    }

    pub fn copy(&self) -> Headers {
        Headers {
            original: self.original.cloned()
        }
    }

    pub fn __iter__(&self) -> HeadersIter {
        HeadersIter {
            original: self.original.to_vec(),
            index: 0
        }
    }

    pub fn __repr__(&self, py: Python<'_>) -> PyResult<String> {
        self.__str__(py)
    }

    pub fn __str__(&self, py: Python<'_>) -> PyResult<String> {
        let list = PyList::empty(py);
        let list_str_any = list.call_method0("__str__")?;
        let list_str = list_str_any.extract::<String>()?;
        Ok(format!("teo.Headers({})", list_str))
    }
}

#[pyclass]
pub struct HeadersIter {
    pub(super) original: Vec<(String, String)>,
    pub(super) index: usize,
}

#[pymethods]
impl HeadersIter {

    fn __next__(&mut self) -> Option<(String, String)> {
        if self.index >= self.original.len() {
            return None;
        }
        let result = self.original.get(self.index)?;
        self.index += 1;
        Some(result.clone())
    }
}