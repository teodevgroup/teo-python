use std::collections::BTreeMap;
use pyo3::{PyObject, PyResult, Python};

pub trait FetchDynamicClasses {

    fn ctxs(&self) -> &BTreeMap<String, PyObject>;

    fn classes(&self) -> &BTreeMap<String, PyObject>;

    fn objects(&self) -> &BTreeMap<String, PyObject>;

    fn ctx(&self, name: &str) -> PyResult<Option<PyObject>> {
        Python::with_gil(|py| {
            Ok(self.ctxs().get(name).map(|o| o.clone_ref(py)))
        })
    }

    fn class(&self, name: &str) -> PyResult<Option<PyObject>> {
        Python::with_gil(|py| {
            Ok(self.classes().get(name).map(|o| o.clone_ref(py)))
        })
    }

    fn object(&self, name: &str) -> PyResult<Option<PyObject>> {
        Python::with_gil(|py| {
            Ok(self.objects().get(name).map(|o| o.clone_ref(py)))
        })
    }
}
