use pyo3::{PyResult, PyObject};
use teo::prelude::r#struct;

pub fn teo_struct_object_to_py_any(_struct_object: &r#struct::Object) -> PyResult<PyObject> {
    unreachable!()
}