use std::collections::HashMap;
use ::teo::prelude::App;
use pyo3::{IntoPy, PyErr, PyObject, PyResult, Python};
use pyo3::exceptions::PyRuntimeError;
use pyo3::types::{PyCFunction, PyDict};
use teo::prelude::{Graph, Object, Value};
use crate::convert::to_teo::py_object_to_teo_value;
use crate::result::IntoPyResult;
use crate::utils::check_py_dict::check_py_dict;

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
    let copy = result.into_py(py);
    classes_mut().insert(name.to_owned(), copy);
    Ok(result.into_py(py))
}

pub fn setup_classes_container() -> PyResult<()> {
    unsafe {
        CLASSES = Some(Box::leak(Box::new(HashMap::new())))
    };
    Ok(())
}

pub fn generate_classes(app: &App, py: Python<'_>) -> PyResult<()> {
    let builtins = py.import("builtins")?;
    let classmethod = builtins.getattr("classmethod")?;
    for model in app.graph().models() {
        let leaked_model_name = Box::leak(Box::new(model.name().to_owned())).as_str();
        let model_heap_class = get_model_class(model.name(), py)?;
        let find_unique = PyCFunction::new_closure(py, Some("find_unique"), Some(INIT_ERROR_MESSAGE), move |args, _kwargs| {
            Python::with_gil(|py| {
                let find_many_arg = if args.len() > 1 {
                    let py_dict = args.get_item(1)?;
                    check_py_dict(py_dict)?;
                    py_object_to_teo_value(py_dict, py)?
                } else {
                    Value::HashMap(HashMap::new())
                };
                let coroutine = pyo3_asyncio::tokio::future_into_py(py, (|| async move {
                    let result = Graph::current().find_unique::<Object>(leaked_model_name, &find_many_arg).await.into_py_result()?;
                    println!("print this: {:?}", result);
                    Ok(())
                })())?;
                Python::with_gil(|py| {
                    Ok::<PyObject, PyErr>(coroutine.into_py(py))
                })
            })
        })?;
        let find_unique_classmethod = classmethod.call1((find_unique,))?;
        model_heap_class.setattr(py, "find_unique", find_unique_classmethod)?;
    }
    Ok(())
}