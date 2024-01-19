use pyo3::{types::PyDict, Python, PyAny, PyResult};
use teo::prelude::Arguments;

use super::teo_object_to_js_any;

pub(crate) fn teo_args_to_py_args(py: Python<'_>, args: &Arguments) -> PyResult<PyDict> {
    let mut dict = PyDict::new(py);
    
    for (k, v) in args.iter() {
        let v = teo_object_to_js_any(v, env)?;
        js_object.set_named_property(k, &v)?;
    }
    Ok(js_object)
}