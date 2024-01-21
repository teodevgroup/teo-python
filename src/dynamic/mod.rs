pub mod model_object_wrapper;
pub mod transaction_ctx_wrapper;
pub mod model_ctx_wrapper;

use std::collections::BTreeMap;
use indexmap::IndexMap;
use inflector::Inflector;
use ::teo::prelude::App;
use pyo3::{IntoPy, PyAny, PyErr, PyObject, PyResult, Python};
use pyo3::exceptions::PyRuntimeError;
use pyo3::types::{PyCFunction, PyDict, PyList};
use teo::prelude::{Namespace, Value, model, transaction};
use crate::dynamic::model_object_wrapper::ModelObjectWrapper;

use crate::object::value::{teo_value_to_py_any, py_any_to_teo_value};
use crate::result::IntoPyResult;
use crate::utils::check_py_dict::check_py_dict;

use self::model_ctx_wrapper::ModelCtxWrapper;
use self::transaction_ctx_wrapper::TransactionCtxWrapper;

static mut CTXS: Option<&'static BTreeMap<String, PyObject>> = None;
static mut CLASSES: Option<&'static BTreeMap<String, PyObject>> = None;
static mut OBJECTS: Option<&'static BTreeMap<String, PyObject>> = None;

pub fn setup_dynamic_container() -> PyResult<()> {
    unsafe { CLASSES = Some(Box::leak(Box::new(BTreeMap::new()))) };
    unsafe { OBJECTS = Some(Box::leak(Box::new(BTreeMap::new()))) };
    unsafe { CTXS = Some(Box::leak(Box::new(BTreeMap::new()))) };
    Ok(())
}

fn classes_mut() -> &'static mut BTreeMap<String, PyObject> {
    unsafe {
        let const_ptr = CLASSES.unwrap() as *const BTreeMap<String, PyObject>;
        let mut_ptr = const_ptr as *mut BTreeMap<String, PyObject>;
        &mut *mut_ptr
    }
}

fn objects_mut() -> &'static mut BTreeMap<String, PyObject> {
    unsafe {
        let const_ptr = OBJECTS.unwrap() as *const BTreeMap<String, PyObject>;
        let mut_ptr = const_ptr as *mut BTreeMap<String, PyObject>;
        &mut *mut_ptr
    }
}

fn ctxs_mut() -> &'static mut BTreeMap<String, PyObject> {
    unsafe {
        let const_ptr = CTXS.unwrap() as *const BTreeMap<String, PyObject>;
        let mut_ptr = const_ptr as *mut BTreeMap<String, PyObject>;
        &mut *mut_ptr
    }
}

pub fn get_model_class_class(py: Python<'_>, name: &str) -> PyResult<PyObject> {
    unsafe {
        if let Some(object_ref) = CLASSES.unwrap().get(name) {
            Ok(object_ref.clone_ref(py))
        } else {
            generate_model_class_class(py, name)
        }
    }
}

pub fn get_model_object_class(py: Python<'_>, name: &str) -> PyResult<PyObject> {
    unsafe {
        if let Some(object_ref) = OBJECTS.unwrap().get(name) {
            Ok(object_ref.clone_ref(py))
        } else {
            generate_model_object_class(py, name)
        }
    }
}

pub fn get_ctx_class(py: Python<'_>, name: &str) -> PyResult<PyObject> {
    unsafe {
        if let Some(object_ref) = CTXS.unwrap().get(name) {
            Ok(object_ref.clone_ref(py))
        } else {
            generate_ctx_class(py, name)
        }
    }
}

pub(crate) fn py_model_class_object_from_teo_model_ctx(py: Python<'_>, model_ctx: model::Ctx, name: &str) -> PyResult<PyObject> {
    let model_name = model_ctx.model.path().join(".");
    let model_class_class = get_model_class_class(py, &model_name)?;
    let model_class_object = model_class_class.call_method1(py, "__new__", (model_class_class,))?;
    model_class_object.setattr(py, "__teo_model_ctx__", ModelCtxWrapper::new(model_ctx))?;
    Ok(model_class_object)
}

pub(crate) fn py_model_object_from_teo_model_object(py: Python<'_>, teo_model_object: model::Object) -> PyResult<PyObject> {
    let model_name = teo_model_object.model().path().join(".");
    let model_object_class = get_model_object_class(py, &model_name)?;
    let model_object = model_object_class.call_method1(py, "__new__", (model_object_class,))?;
    model_object.setattr(py, "__teo_object__", ModelObjectWrapper::new(teo_model_object))?;
    Ok(model_object)
}

pub(crate) fn py_optional_model_object_from_teo_object(py: Python<'_>, teo_model_object: Option<model::Object>) -> PyResult<PyObject> {
    Ok(match teo_model_object {
        Some(teo_model_object) => py_model_object_from_teo_model_object(py, teo_model_object)?,
        None => ().into_py(py),
    })
}

pub(crate) fn py_ctx_object_from_teo_transaction_ctx(py: Python<'_>, transaction_ctx: transaction::Ctx, name: &str) -> PyResult<PyObject> {
    let ctx_class = get_ctx_class(py, name)?;
    let ctx_object = ctx_class.call_method1(py, "__new__", (ctx_class,))?;
    ctx_object.setattr(py, "__teo_transaction_ctx__", TransactionCtxWrapper::new(transaction_ctx))?;
    Ok(ctx_object)
}

pub(crate) fn teo_model_ctx_from_py_model_class_object(py: Python<'_>, model_class_object: PyObject) -> PyResult<model::Ctx> {
    let wrapper: &ModelCtxWrapper = model_class_object.getattr(py, "__teo_model_ctx__")?.extract(py)?;
    Ok(wrapper.ctx.clone())
}

pub(crate) fn teo_model_object_from_py_model_object(py: Python<'_>, model_class_object: PyObject) -> PyResult<model::Object> {
    let wrapper: &ModelObjectWrapper = model_class_object.getattr(py, "__teo_object__")?.extract(py)?;
    Ok(wrapper.object.clone())
}

pub(crate) fn teo_transaction_ctx_from_py_ctx_object(py: Python<'_>, ctx_object: PyObject) -> PyResult<transaction::Ctx> {
    let wrapper: &TransactionCtxWrapper = ctx_object.getattr(py, "__teo_transaction_ctx__")?.extract(py)?;
    Ok(wrapper.ctx.clone())
}

static INIT_ERROR_MESSAGE: &str = "class is not initialized";

unsafe fn generate_model_class_class(py: Python<'_>, name: &str) -> PyResult<PyObject> {
    let builtins = py.import("builtins")?;
    let py_type = builtins.getattr("type")?;
    let py_object = builtins.getattr("object")?;
    let dict = PyDict::new(py);
    dict.set_item("__module__", "teo.models")?;
    let init = PyCFunction::new_closure(py, Some("__init__"), Some(INIT_ERROR_MESSAGE), |args, _kwargs| {
        let slf = args.get_item(0)?;
        let initialized: bool = slf.getattr("__teo_initialized__")?.extract()?;
        if initialized {
            Ok(())
        } else {
            Err::<(), PyErr>(PyRuntimeError::new_err(INIT_ERROR_MESSAGE))
        }
    })?;
    dict.set_item("__init__", init)?;
    let result = py_type.call1((name, (py_object,), dict))?;
    let result_object = result.into_py(py);
    classes_mut().insert(name.to_owned(), result_object);
    Ok(result.into_py(py))
}

unsafe fn generate_model_object_class(py: Python<'_>, name: &str) -> PyResult<PyObject> {
    let builtins = py.import("builtins")?;
    let py_type = builtins.getattr("type")?;
    let py_object = builtins.getattr("object")?;
    let dict = PyDict::new(py);
    dict.set_item("__module__", "teo.models")?;
    let init = PyCFunction::new_closure(py, Some("__init__"), Some(INIT_ERROR_MESSAGE), |args, _kwargs| {
        let slf = args.get_item(0)?;
        let initialized: bool = slf.getattr("__teo_initialized__")?.extract()?;
        if initialized {
            Ok(())
        } else {
            Err::<(), PyErr>(PyRuntimeError::new_err(INIT_ERROR_MESSAGE))
        }
    })?;
    dict.set_item("__init__", init)?;
    let result = py_type.call1((name, (py_object,), dict))?;
    let result_object = result.into_py(py);
    objects_mut().insert(name.to_owned(), result_object);
    Ok(result.into_py(py))
}

unsafe fn generate_ctx_class(py: Python<'_>, name: &str) -> PyResult<PyObject> {
    let builtins = py.import("builtins")?;
    let py_type = builtins.getattr("type")?;
    let py_object = builtins.getattr("object")?;
    let dict = PyDict::new(py);
    dict.set_item("__module__", "teo.models")?;
    let init = PyCFunction::new_closure(py, Some("__init__"), None, |args, _kwargs| {
        let slf = args.get_item(0)?;
        let initialized: bool = slf.getattr("__teo_initialized__")?.extract()?;
        if initialized {
            Ok(())
        } else {
            Err::<(), PyErr>(PyRuntimeError::new_err(INIT_ERROR_MESSAGE))
        }
    })?;
    dict.set_item("__init__", init)?;
    let result = py_type.call1((name, (py_object,), dict))?;
    let result_object = result.into_py(py);
    ctxs_mut().insert(name.to_owned(), result_object);
    Ok(result.into_py(py))
}

pub(crate) fn synthesize_dynamic_python_classes(py: Python<'_>, app: &App) -> PyResult<()> {
    synthesize_dynamic_nodejs_classes_for_namespace(py, app.main_namespace())
}

pub(crate) fn synthesize_dynamic_nodejs_classes_for_namespace(py: Python<'_>, namespace: &'static Namespace) -> PyResult<()> {
    synthesize_direct_dynamic_nodejs_classes_for_namespace(py, namespace)?;
    for namespace in namespace.namespaces.values() {
        synthesize_dynamic_nodejs_classes_for_namespace(py, namespace)?;
    }
    Ok(())
}

fn synthesize_direct_dynamic_nodejs_classes_for_namespace(py: Python<'_>, namespace: &'static Namespace) -> PyResult<()> {
    let builtins = py.import("builtins")?;
    let property_wrapper = builtins.getattr("property")?;
    let ctx_class = get_ctx_class(py, &namespace.path().join("."))?;
    for model in namespace.models.values() {
        let model_name = model.path().join(".");
        let model_property_name = model.path().last().unwrap().to_snake_case();
        let model_property = PyCFunction::new_closure(py, model_name, None, move |args, _kwargs| {
            let slf = args.get_item(0)?;
            let transaction_ctx_wrapper: &TransactionCtxWrapper = slf.getattr("__teo_transaction_ctx__")?.extract()?;
            let model_ctx = transaction_ctx_wrapper.ctx.model_ctx_for_model_at_path(&model.path()).unwrap();
            let model_class_class = get_model_class_class(py, &model_name)?;
            let model_class_object = model_class_class.call_method1(py, "__new__", (model_class_class.as_ref(py),))?;
            model_class_object.setattr(py, "__teo_model_ctx__", ModelCtxWrapper::new(model_ctx))?;
            Ok(model_class_object)
        })?;
        let model_property_wrapped = property_wrapper.call1((model_property,))?;
        ctx_class.setattr(py, model_property_name, model_property_wrapped)?;
        let model_class_class = get_model_class_class(py, &model_name)?;
        // find unique
        let find_unique = find_unique_function(&model_name, py)?;
        model_class_class.setattr(py, "find_unique", find_unique)?;
        // find first
        let find_first = find_first_function(&model_name, py)?;
        model_class_class.setattr(py, "find_first", find_first)?;
        // find many
        let find_many = find_many_function(&model_name, py)?;
        model_class_class.setattr(py, "find_many", find_many)?;
        // __repr__
        let repr = repr_function(&model_name, model_class_class.as_ref(py), py)?;
        model_class_class.setattr(py, "__repr__", repr)?;
        let model_object_class = get_model_object_class(py, &model_name)?;

    }
    for namespace in namespace.namespaces.values() {
        let namespace_name = namespace.path().join(".");
        let namespace_property = PyCFunction::new_closure(py, namespace_name, None, move |args, _kwargs| {
            let slf = args.get_item(0)?;
            let transaction_ctx_wrapper: &TransactionCtxWrapper = slf.getattr("__teo_transaction_ctx__")?.extract()?;
            let next_ctx_class = get_ctx_class(py, &namespace_name)?;
            let next_ctx_object = next_ctx_class.call_method1(py, "__new__", (next_ctx_class.as_ref(py),))?;
            next_ctx_object.setattr(py, "__teo_transaction_ctx__", transaction_ctx_wrapper.clone())?;
            Ok(next_ctx_object)
        })?;
        let namespace_property_wrapped = property_wrapper.call1((namespace_property,))?;
        ctx_class.setattr(py, namespace.path().last().unwrap().to_snake_case(), namespace_property_wrapped)?;
    }
    // TODO: transaction
    ctx_class.setattr(py, "__teo_initialized__", true)?;
    Ok(())
}


fn find_unique_function<'py>(model_name: &'static str, py: Python<'py>) -> PyResult<&'py PyCFunction> {
    Ok(PyCFunction::new_closure(py, Some("find_unique"), Some("Find a unique record."), move |args, _kwargs| {
        Python::with_gil(|py| {
            let slf = args.get_item(0)?.into_py(py);
            let model_ctx_wrapper: ModelCtxWrapper = slf.getattr(py, "__teo_model_ctx__")?.extract(py)?;
            let find_many_arg = if args.len() > 1 {
                let py_dict = args.get_item(1)?;
                check_py_dict(py_dict)?;
                py_any_to_teo_value(py, py_dict)?
            } else {
                Value::Dictionary(IndexMap::new())
            };
            let coroutine = pyo3_asyncio::tokio::future_into_py(py, (|| async move {
                let result: Option<model::Object> = model_ctx_wrapper.ctx.find_unique(&find_many_arg).await.into_py_result()?;
                Python::with_gil(|py| {
                    match result {
                        Some(object) => {
                            py_model_object_from_teo_model_object(py, object)
                        }
                        None => {
                            Ok(().into_py(py))
                        }
                    }
                })
            })())?;
            Ok::<PyObject, PyErr>(coroutine.into_py(py))
        })
    })?)
}

fn find_first_function<'py>(model_name: &'static str, py: Python<'py>) -> PyResult<&'py PyCFunction> {
    Ok(PyCFunction::new_closure(py, Some("find_first"), Some("Find a record."), move |args, _kwargs| {
        Python::with_gil(|py| {
            let slf = args.get_item(0)?.into_py(py);
            let model_ctx_wrapper: ModelCtxWrapper = slf.getattr(py, "__teo_model_ctx__")?.extract(py)?;
            let find_many_arg = if args.len() > 1 {
                let py_dict = args.get_item(1)?;
                check_py_dict(py_dict)?;
                py_any_to_teo_value(py, py_dict)?
            } else {
                Value::Dictionary(IndexMap::new())
            };
            let coroutine = pyo3_asyncio::tokio::future_into_py(py, (|| async move {
                let result: Option<model::Object> = model_ctx_wrapper.ctx.find_first(&find_many_arg).await.into_py_result()?;
                Python::with_gil(|py| {
                    match result {
                        Some(object) => {
                            py_model_object_from_teo_model_object(py, object)
                        }
                        None => {
                            Ok(().into_py(py))
                        }
                    }
                })
            })())?;
            Ok::<PyObject, PyErr>(coroutine.into_py(py))
        })
    })?)
}

fn find_many_function<'py>(model_name: &'static str, py: Python<'py>) -> PyResult<&'py PyCFunction> {
    Ok(PyCFunction::new_closure(py, Some("find_many"), Some("Find many records."), move |args, _kwargs| {
        Python::with_gil(|py| {
            let slf = args.get_item(0)?.into_py(py);
            let model_ctx_wrapper: ModelCtxWrapper = slf.getattr(py, "__teo_model_ctx__")?.extract(py)?;
            let find_many_arg = if args.len() > 1 {
                let py_dict = args.get_item(1)?;
                check_py_dict(py_dict)?;
                py_any_to_teo_value(py, py_dict)?
            } else {
                Value::Dictionary(IndexMap::new())
            };
            let coroutine = pyo3_asyncio::tokio::future_into_py::<_, PyObject>(py, (|| async move {
                let result: Vec<model::Object> = model_ctx_wrapper.ctx.find_many(&find_many_arg).await.into_py_result()?;
                Python::with_gil(|py| {
                    let py_result = PyList::empty(py);
                    for object in result {
                        let instance = py_model_object_from_teo_model_object(py, object)?;
                        py_result.append(instance)?;
                    }
                    Ok(py_result.into_py(py))
                })
            })())?;
            Ok::<PyObject, PyErr>(coroutine.into_py(py))
        })
    })?)
}

fn repr_function<'py>(model_name: &'static str, cls: &'py PyAny, py: Python<'py>) -> PyResult<&'py PyCFunction> {
    let function_name = Box::leak(Box::new(format!("{model_name}.__repr__"))).as_str();
    Ok(PyCFunction::new_method_closure(py, cls, Some(function_name), Some("Represent."), move |args, kwargs| {
        Python::with_gil(|py| {
            println!("see args and kwargs: {:?} {:?}", args, kwargs);
            let slf = args.get_item(0)?;
            let object_wrapper: ModelObjectWrapper = slf.getattr("__teo_object__")?.extract()?;
            let object = &object_wrapper.object;
            let mut fields = "".to_owned();
            for (index, field) in object.model().fields().iter().enumerate() {
                if index != 0 {
                    fields += ", ";
                }
                let value = object.get_value(field.name()).into_py_result(py)?;
                let py_object = teo_value_to_py_any(py, &value)?;
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