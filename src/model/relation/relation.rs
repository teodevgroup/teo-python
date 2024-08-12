use pyo3::{pyclass, pymethods, types::PyAnyMethods, IntoPy, PyAny, PyObject, PyResult, Python};
use teo::prelude::model;

use crate::{dynamic::py_class_lookup_map::PYClassLookupMap, object::value::{py_any_to_teo_value, teo_value_to_py_any}};

#[pyclass]
pub struct Relation {
    pub(crate) teo_relation: model::relation::Builder,
}

#[pymethods]
impl Relation {

    pub fn set_data(&mut self, py: Python<'_>, key: String, value: PyObject) -> PyResult<()> {
        self.teo_relation.data().insert(key, py_any_to_teo_value(py, value.bind(py).extract::<&PyAny>()?)?);
        Ok(())
    }

    pub fn data(&mut self, py: Python<'_>, key: String) -> PyResult<PyObject> {
        Ok(match self.teo_relation.data().get(key.as_str()) {
            Some(object) => teo_value_to_py_any(py, object, PYClassLookupMap::from_app_data(self.teo_relation.app_data()))?,
            None => ().into_py(py),
        })
    }
}