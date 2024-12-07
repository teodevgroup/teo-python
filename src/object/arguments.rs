use pyo3::{types::{PyDict, PyDictMethods}, PyObject, PyResult, Python};
use teo::prelude::Arguments;
use super::value::teo_value_to_py_any_without_model_objects;

pub(crate) fn teo_args_to_py_args(py: Python<'_>, args: &Arguments) -> PyResult<PyObject> {
    let dict = PyDict::new(py);
    for (k, v) in args.iter() {
        let v = teo_value_to_py_any_without_model_objects(py, v)?;
        dict.set_item(k, &v)?;
    }
    Ok(dict.into_any().unbind())
}
