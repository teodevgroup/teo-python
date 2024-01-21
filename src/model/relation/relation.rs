use pyo3::{PyResult, Python, pyclass, pymethods, IntoPy, PyObject};
use teo::prelude::model::Relation as TeoRelation;

use crate::object::{teo_object_to_py_any, py_any_to_teo_object};

#[pyclass]
pub struct Relation {
    pub(crate) teo_relation: &'static mut TeoRelation,
}

#[pymethods]
impl Relation {

    pub fn set_data(&mut self, py: Python<'_>, key: String, value: PyObject) -> PyResult<()> {
        self.teo_relation.data.insert(key, py_any_to_teo_object(py, value)?);
        Ok(())
    }

    pub fn data(&mut self, py: Python<'_>, key: String) -> PyResult<PyObject> {
        Ok(match self.teo_relation.data.get(key.as_str()) {
            Some(object) => teo_object_to_py_any(py, object)?,
            None => ().into_py(py),
        })
    }
}