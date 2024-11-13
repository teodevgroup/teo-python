use pyo3::{types::{PyDict, PyDictMethods}, IntoPy, PyObject, PyResult, Python};
use teo::prelude::Arguments;
use crate::dynamic::py_class_lookup_map::PYClassLookupMap;

use super::value::teo_value_to_py_any;

pub(crate) fn teo_args_to_py_args(py: Python<'_>, args: &Arguments, map: &PYClassLookupMap) -> PyResult<PyObject> {
    let dict = PyDict::new_bound(py);
    for (k, v) in args.iter() {
        let v = teo_value_to_py_any(py, v, map)?;
        dict.set_item(k, &v)?;
    }
    Ok(dict.into_py(py))
}