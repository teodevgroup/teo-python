use pyo3::{pyclass, pymethods, Python, PyObject, PyResult, IntoPy};
use teo::prelude::Member as TeoEnumMember;

use crate::object::{teo_object_to_py_any, py_any_to_teo_object};

#[pyclass]
pub struct EnumMember {
    pub(crate) teo_enum_member: &'static mut TeoEnumMember,
}

#[pymethods]
impl EnumMember {

    pub fn set_data(&mut self, py: Python<'_>, key: String, value: PyObject) -> PyResult<()> {
        self.teo_enum_member.data.insert(key, py_any_to_teo_object(py, value)?);
        Ok(())
    }

    pub fn data(&mut self, py: Python<'_>, key: String) -> PyResult<PyObject> {
        Ok(match self.teo_enum_member.data.get(key.as_str()) {
            Some(object) => teo_object_to_py_any(py, object)?,
            None => ().into_py(py),
        })
    }
}