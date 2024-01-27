use pyo3::{types::PyDict, Python, PyResult, PyObject, IntoPy};
use teo::prelude::Arguments;

use super::teo_object_to_py_any;

pub(crate) fn teo_args_to_py_args(py: Python<'_>, args: &Arguments) -> PyResult<PyObject> {
    let dict = PyDict::new(py);
    for (k, v) in args.iter() {
        let v = teo_object_to_py_any(py, v)?;
        dict.set_item(k, &v)?;
    }
    Ok(dict.into_py(py))
}