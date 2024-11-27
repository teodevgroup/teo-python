use pyo3::{pyclass, pymethods, types::PyNone, Bound, PyAny, PyObject, PyResult, Python};
use teo::prelude::r#enum;

use crate::{dynamic::py_class_lookup_map::PYClassLookupMap, object::value::{py_any_to_teo_value, teo_value_to_py_any}};

#[pyclass]
pub struct EnumMember {
    pub(crate) teo_enum_member: r#enum::member::Builder,
}

#[pymethods]
impl EnumMember {

    pub fn set_data(&mut self, py: Python<'_>, key: String, value: &Bound<PyAny>) -> PyResult<()> {
        self.teo_enum_member.data().insert(key, py_any_to_teo_value(py, value)?);
        Ok(())
    }

    pub fn data(&mut self, py: Python<'_>, key: String) -> PyResult<PyObject> {
        Ok(match self.teo_enum_member.data().get(key.as_str()) {
            Some(object) => teo_value_to_py_any(py, object, PYClassLookupMap::from_app_data(self.teo_enum_member.app_data()))?,
            None => PyNone::get(py).as_unbound().clone_ref(py).into_any(),
        })
    }
}