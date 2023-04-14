use std::collections::HashMap;
use ::teo::prelude::App;
use pyo3::{IntoPy, PyErr, PyObject, PyResult, Python};
use pyo3::exceptions::PyRuntimeError;
use pyo3::types::{PyCFunction, PyDict};

static mut CLASSES: Option<&'static HashMap<String, PyObject>> = None;

fn classes_mut() -> &'static mut HashMap<String, PyObject> {
    unsafe {
        let const_ptr = CLASSES.unwrap() as *const HashMap<String, PyObject>;
        let mut_ptr = const_ptr as *mut HashMap<String, PyObject>;
        &mut *mut_ptr
    }
}

pub fn get_model_class(name: &str, py: Python<'_>) -> PyResult<PyObject> {
    unsafe {
        if let Some(object_ref) = CLASSES.unwrap().get(name) {
            Ok(object_ref.clone_ref(py))
        } else {
            generate_model_class(name, py)
        }
    }
}

static INIT_ERROR_MESSAGE: &str = "Do not call Teo model init directly. Use `create' instead.";

unsafe fn generate_model_class(name: &str, py: Python<'_>) -> PyResult<PyObject> {
    let builtins = py.import("builtins")?;
    let py_type = builtins.getattr("type")?;
    let py_object = builtins.getattr("object")?;
    let dict = PyDict::new(py);
    dict.set_item("__module__", "teo.models")?;
    let init = PyCFunction::new_closure(py, Some("__init__"), Some(INIT_ERROR_MESSAGE), |_slf, _args| {
        Err::<(), PyErr>(PyRuntimeError::new_err(INIT_ERROR_MESSAGE))
    })?;
    dict.set_item("__init__", init)?;
    let result = py_type.call1((name, (py_object,), dict))?;
    Ok(result.into_py(py))
}

pub fn setup_classes_container() -> PyResult<()> {
    unsafe {
        CLASSES = Some(Box::leak(Box::new(HashMap::new())))
    };
    Ok(())
}

pub fn generate_classes(app: &App, _py: Python<'_>) -> PyResult<()> {
    for _model in app.graph().models() {

    }
    Ok(())
}