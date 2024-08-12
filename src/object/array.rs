use pyo3::{Python, PyResult, PyObject, types::PyList, IntoPy};
use teo::prelude::Value;
use crate::dynamic::py_class_lookup_map::PYClassLookupMap;

use super::value::teo_value_to_py_any;

pub fn teo_array_to_py_any(py: Python<'_>, array: &Vec<Value>, map: &PYClassLookupMap) -> PyResult<PyObject> {
    let list = PyList::empty(py);
    for (_, value) in array.iter().enumerate() {
        let v = teo_value_to_py_any(py, value, map)?;
        list.append(v)?;
    }
    Ok(list.into_py(py))
}