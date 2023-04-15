use std::collections::HashMap;
use ::teo::prelude::App;
use pyo3::{AsPyPointer, IntoPy, PyAny, PyErr, PyMethodType, PyObject, PyResult, Python};
use pyo3::exceptions::PyRuntimeError;
use pyo3::ffi::{PyCMethod, PyCMethod_New};
use pyo3::methods::OkWrap;
use pyo3::types::{PyCFunction, PyDict, PyList};
use teo::prelude::{Graph, Object, Value};
use crate::classes::object_wrapper::ObjectWrapper;
use crate::convert::to_py::teo_value_to_py_object;
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
        let model_name = Box::leak(Box::new(model.name().to_owned())).as_str();
        let model_heap_class = get_model_class(model.name(), py)?;
        // find unique
        let find_unique = find_unique_function(model_name, py)?;
        let find_unique_classmethod = classmethod.call1((find_unique,))?;
        model_heap_class.setattr(py, "find_unique", find_unique_classmethod)?;
        // find first
        let find_first = find_first_function(model_name, py)?;
        let find_first_classmethod = classmethod.call1((find_first,))?;
        model_heap_class.setattr(py, "find_first", find_first_classmethod)?;
        // find many
        let find_many = find_many_function(model_name, py)?;
        let find_many_classmethod = classmethod.call1((find_many,))?;
        model_heap_class.setattr(py, "find_many", find_many_classmethod)?;
        // __repr__
        let repr = repr_function(model_name, model_heap_class.as_ref(py), py)?;
        model_heap_class.setattr(py, "__repr__", repr)?;
    }
    Ok(())
}

fn find_unique_function<'py>(model_name: &'static str, py: Python<'py>) -> PyResult<&'py PyCFunction> {
    Ok(PyCFunction::new_closure(py, Some("find_unique"), Some("Find a unique record."), move |args, _kwargs| {
        Python::with_gil(|py| {
            let cls = args.get_item(0)?.into_py(py);
            let find_many_arg = if args.len() > 1 {
                let py_dict = args.get_item(1)?;
                check_py_dict(py_dict)?;
                py_object_to_teo_value(py_dict, py)?
            } else {
                Value::HashMap(HashMap::new())
            };
            let coroutine = pyo3_asyncio::tokio::future_into_py(py, (|| async move {
                let result = Graph::current().find_unique::<Object>(model_name, &find_many_arg).await.into_py_result()?;
                Python::with_gil(|py| {
                    match result {
                        Some(object) => {
                            let instance = cls.call_method1(py, "__new__", (cls.as_ref(py),))?;
                            instance.setattr(py, "__teo_object__", ObjectWrapper::new(object))?;
                            Ok(instance)
                        }
                        None => {
                            Ok(().into_py(py))
                        }
                    }
                })
            })())?;
            Python::with_gil(|py| {
                Ok::<PyObject, PyErr>(coroutine.into_py(py))
            })
        })
    })?)
}

fn find_first_function<'py>(model_name: &'static str, py: Python<'py>) -> PyResult<&'py PyCFunction> {
    Ok(PyCFunction::new_closure(py, Some("find_first"), Some("Find a record."), move |args, _kwargs| {
        Python::with_gil(|py| {
            let cls = args.get_item(0)?.into_py(py);
            let find_many_arg = if args.len() > 1 {
                let py_dict = args.get_item(1)?;
                check_py_dict(py_dict)?;
                py_object_to_teo_value(py_dict, py)?
            } else {
                Value::HashMap(HashMap::new())
            };
            let coroutine = pyo3_asyncio::tokio::future_into_py(py, (|| async move {
                let result = Graph::current().find_first::<Object>(model_name, &find_many_arg).await.into_py_result()?;
                Python::with_gil(|py| {
                    match result {
                        Some(object) => {
                            let instance = cls.call_method1(py, "__new__", (cls.as_ref(py),))?;
                            instance.setattr(py, "__teo_object__", ObjectWrapper::new(object))?;
                            Ok(instance)
                        }
                        None => {
                            Ok(().into_py(py))
                        }
                    }
                })
            })())?;
            Python::with_gil(|py| {
                Ok::<PyObject, PyErr>(coroutine.into_py(py))
            })
        })
    })?)
}

fn find_many_function<'py>(model_name: &'static str, py: Python<'py>) -> PyResult<&'py PyCFunction> {
    Ok(PyCFunction::new_closure(py, Some("find_many"), Some("Find many records."), move |args, _kwargs| {
        Python::with_gil(|py| {
            let cls = args.get_item(0)?.into_py(py);
            let find_many_arg = if args.len() > 1 {
                let py_dict = args.get_item(1)?;
                check_py_dict(py_dict)?;
                py_object_to_teo_value(py_dict, py)?
            } else {
                Value::HashMap(HashMap::new())
            };
            let coroutine = pyo3_asyncio::tokio::future_into_py::<_, PyObject>(py, (|| async move {
                let result = Graph::current().find_many::<Object>(model_name, &find_many_arg).await.into_py_result()?;
                Python::with_gil(|py| {
                    let py_result = PyList::empty(py);
                    for object in result {
                        let instance = cls.call_method1(py, "__new__", (cls.as_ref(py),))?;
                        instance.setattr(py, "__teo_object__", ObjectWrapper::new(object))?;
                        py_result.append(instance)?;
                    }
                    Ok(py_result.into_py(py))
                })
            })())?;
            Python::with_gil(|py| {
                Ok::<PyObject, PyErr>(coroutine.into_py(py))
            })
        })
    })?)
}

fn repr_function<'py>(model_name: &'static str, cls: &'py PyAny, py: Python<'py>) -> PyResult<&'py PyCFunction> {
    let function_name = Box::leak(Box::new(format!("{model_name}.__repr__"))).as_str();
    Ok(PyCFunction::new_method_closure(py, cls, Some(function_name), Some("Represent."), move |args, kwargs| {
        Python::with_gil(|py| {
            println!("see args and kwargs: {:?} {:?}", args, kwargs);
            let slf = args.get_item(0)?;
            let object_wrapper: ObjectWrapper = slf.getattr("__teo_object__")?.extract()?;
            let object = &object_wrapper.object;
            let mut fields = "".to_owned();
            for (index, field) in object.model().fields().iter().enumerate() {
                if index != 0 {
                    fields += ", ";
                }
                let value = object.get_value(field.name()).into_py_result()?;
                let py_object = teo_value_to_py_object(value, py)?;
                let py_repr_result = py_object.call_method0(py, "__repr__")?;
                let inner_py_repr: &str = py_repr_result.extract(py)?;
                fields += field.name();
                fields += "=";
                fields += inner_py_repr;
            }
            let result = format!("{}({})", object.model().name(), fields);
            Ok::<String, PyErr>(result)
        })
    })?)
}

//__repr__