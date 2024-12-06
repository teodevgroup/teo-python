use pyo3::{types::{PyList, PyListMethods}, PyObject, PyResult, Python};
use teo::prelude::Value;
use crate::dynamic::DynamicClasses;
use super::value::teo_value_to_py_any;

pub(crate) fn teo_array_to_py_any(py: Python<'_>, array: &Vec<Value>, map: &DynamicClasses) -> PyResult<PyObject> {
    let list = PyList::empty(py);
    for (_, value) in array.iter().enumerate() {
        let v = teo_value_to_py_any(py, value, map)?;
        list.append(v)?;
    }
    Ok(list.unbind().into_any())
}