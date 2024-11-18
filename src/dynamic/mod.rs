pub mod model_object_wrapper;
pub mod transaction_ctx_wrapper;
pub mod model_ctx_wrapper;
pub mod py_class_lookup_map;

use std::ffi::CString;
use std::sync::Arc;
use indexmap::IndexMap;
use inflector::Inflector;
use py_class_lookup_map::PYClassLookupMap;
use pyo3_async_runtimes::TaskLocals;
use teo::prelude::app::data::AppData;
use teo::prelude::traits::named::Named;
use ::teo::prelude::App;
use pyo3::{Bound, IntoPy, PyAny, PyErr, PyObject, PyResult, Python};
use pyo3::types::{PyAnyMethods, PyCFunction, PyDict, PyList, PyListMethods};
use teo::prelude::{Namespace, Value, model, transaction};
use crate::dynamic::model_object_wrapper::ModelObjectWrapper;

use crate::object::model::teo_model_object_to_py_any;
use crate::object::value::{teo_value_to_py_any, py_any_to_teo_value};
use crate::utils::await_coroutine_if_needed::await_coroutine_if_needed_value_with_locals;
use crate::utils::check_py_dict::check_py_dict;

use self::model_ctx_wrapper::ModelCtxWrapper;
use self::transaction_ctx_wrapper::TransactionCtxWrapper;

pub(crate) fn synthesize_dynamic_python_classes(app: &App, py: Python<'_>) -> PyResult<()> {
    let static_app = unsafe { &*(app as *const App) } as &'static App;
    let mut map = PYClassLookupMap::new();
    synthesize_dynamic_python_classes_for_namespace(&mut map, static_app, static_app.compiled_main_namespace(), py)?;
    let raw_map_pointer = Box::into_raw(Box::new(map));
    app.app_data().set_dynamic_classes_pointer(raw_map_pointer as * mut ());
    app.app_data().set_dynamic_classes_clean_up(Arc::new(|app_data: AppData| {
        unsafe {
            let raw_pointer = app_data.dynamic_classes_pointer() as * mut PYClassLookupMap;
            let _ = Box::from_raw(raw_pointer);
        }
    }));
    Ok(())
}

pub(crate) fn synthesize_dynamic_python_classes_for_namespace(map: &mut PYClassLookupMap, app: &'static App, namespace: &'static Namespace, py: Python<'_>) -> PyResult<()> {
    synthesize_direct_dynamic_python_classes_for_namespace(map, app, namespace, py)?;
    for namespace in namespace.namespaces().values() {
        synthesize_dynamic_python_classes_for_namespace(map, app, namespace, py)?;
    }
    Ok(())
}

pub(crate) fn teo_model_ctx_from_py_model_class_object(py: Python<'_>, model_class_object: PyObject) -> PyResult<model::Ctx> {
    let wrapper: ModelCtxWrapper = model_class_object.getattr(py, "__teo_model_ctx__")?.extract(py)?;
    Ok(wrapper.ctx.clone())
}

pub(crate) fn teo_model_object_from_py_model_object(py: Python<'_>, model_object: PyObject) -> PyResult<model::Object> {
    let wrapper: ModelObjectWrapper = model_object.getattr(py, "__teo_object__")?.extract(py)?;
    Ok(wrapper.object.clone())
}

pub(crate) fn teo_transaction_ctx_from_py_ctx_object(py: Python<'_>, ctx_object: PyObject) -> PyResult<transaction::Ctx> {
    let wrapper: TransactionCtxWrapper = ctx_object.getattr(py, "__teo_transaction_ctx__")?.extract(py)?;
    Ok(wrapper.ctx.clone())
}

static INIT_ERROR_MESSAGE: &str = "class is not initialized";

pub(crate) fn synthesize_direct_dynamic_python_classes_for_namespace(map: &mut PYClassLookupMap, app: &'static App, namespace: &'static Namespace, py: Python<'_>) -> PyResult<()> {
    let app_data = app.app_data();
    let main = py.import_bound("__main__")?;
    let teo_wrap_builtin = main.getattr("teo_wrap_builtin")?;
    let teo_wrap_async = main.getattr("teo_wrap_async")?;
    let builtins = py.import_bound("builtins")?;
    let property_wrapper = builtins.getattr("property")?;
    let ctx_class = map.ctx_or_create(&namespace.path().join("."), py)?;
    for model in namespace.models().values() {
        let model_name = Box::leak(Box::new(model.path().join("."))).as_str();
        let model_name_c = Box::leak(Box::new(CString::new(model_name)?)).as_c_str();
        let model_property_name = model.path().last().unwrap().to_snake_case();
        let model_property = PyCFunction::new_closure_bound(py, Some(model_name_c), None, move |args, _kwargs| {
            let model_class_object = Python::with_gil(|py| {
                let map = PYClassLookupMap::from_app_data(app_data);
                let slf = args.get_item(0)?;
                let transaction_ctx_wrapper: TransactionCtxWrapper = slf.getattr("__teo_transaction_ctx__")?.extract()?;
                let model_ctx = transaction_ctx_wrapper.ctx.model_ctx_for_model_at_path(&model.path()).unwrap();
                let model_class_class = map.class(&model_name)?.unwrap();
                let model_class_object = model_class_class.call_method1(py, "__new__", (model_class_class.clone_ref(py),))?;
                model_class_object.setattr(py, "__teo_model_ctx__", ModelCtxWrapper::new(model_ctx))?;
                Ok::<PyObject, PyErr>(model_class_object)
            })?;
            Ok::<PyObject, PyErr>(model_class_object)
        })?;
        let model_property_wrapped = property_wrapper.call1((model_property,))?;
        ctx_class.setattr(py, model_property_name.as_str(), model_property_wrapped)?;
        // class object methods
        let model_class_class = map.class_or_create(&model_name, py)?;
        // find unique
        let find_unique_object = find_unique_object_function(py, app_data)?;
        teo_wrap_builtin.call1((model_class_class.bind(py), "find_unique_object", teo_wrap_async.call1((find_unique_object,))?))?;
        // find first
        let find_first_object = find_first_object_function(py, app_data)?;
        teo_wrap_builtin.call1((model_class_class.bind(py), "find_first_object", teo_wrap_async.call1((find_first_object,))?))?;
        // find many
        let find_many_objects = find_many_objects_function(py, app_data)?;
        teo_wrap_builtin.call1((model_class_class.bind(py), "find_many_objects", teo_wrap_async.call1((find_many_objects,))?))?;
        // create
        let create_object: Bound<'_, PyCFunction> = create_object_function(py, app_data)?;
        teo_wrap_builtin.call1((model_class_class.bind(py), "create_object", teo_wrap_async.call1((create_object,))?))?;
        // count objects
        let count_objects = count_objects_function(py)?;
        teo_wrap_builtin.call1((model_class_class.bind(py), "count_objects", count_objects))?;
        // count fields
        let count_fields = count_fields_function(py, app_data)?;
        teo_wrap_builtin.call1((model_class_class.bind(py), "count_fields", count_fields))?;
        // aggregate
        let aggregate = aggregate_function(py, app_data)?;
        teo_wrap_builtin.call1((model_class_class.bind(py), "aggregate", aggregate))?;
        // group by
        let group_by = group_by_function(py, app_data)?;
        teo_wrap_builtin.call1((model_class_class.bind(py), "group_by", group_by))?;
        // sql
        if namespace.database().is_some() && namespace.database().unwrap().is_sql() {
            let sql = sql_function(py, app_data)?;
            teo_wrap_builtin.call1((model_class_class.bind(py), "sql", sql))?;    
        }
        // model object methods
        let model_object_class = map.object_or_create(&model_name, py)?;
        // is new
        let is_new = is_new_function(py)?;
        teo_wrap_builtin.call1((model_object_class.bind(py), "is_new", is_new))?;
        // is modified
        let is_modified = is_modified_function(py)?;
        teo_wrap_builtin.call1((model_object_class.bind(py), "is_modified", is_modified))?;
        // set
        let set = set_function(py)?;
        teo_wrap_builtin.call1((model_object_class.bind(py), "set", set))?;
        // update
        let update = update_function(py)?;
        teo_wrap_builtin.call1((model_object_class.bind(py), "update", update))?;
        // save
        let save = save_function(py)?;
        teo_wrap_builtin.call1((model_object_class.bind(py), "save", save))?;
        // delete
        let delete = delete_function(py)?;
        teo_wrap_builtin.call1((model_object_class.bind(py), "delete", delete))?;
        // to teon
        let to_teon = to_teon_function(py, app_data)?;
        teo_wrap_builtin.call1((model_object_class.bind(py), "to_teon", to_teon))?;
        // __repr__
        let repr = repr_function(py, app_data)?;
        teo_wrap_builtin.call1((model_object_class.bind(py), "__repr__", repr))?;
        // fields
        for field in model.fields().values() {
            let snake_case_field_name = Box::leak(Box::new(field.name().to_snake_case())).as_str();
            let snake_case_field_name_c = Box::leak(Box::new(CString::new(snake_case_field_name)?)).as_c_str();
            let field_name = Box::leak(Box::new(field.name().to_string())).as_str();
            let field_property_getter = PyCFunction::new_closure_bound(py, Some(snake_case_field_name_c), None, move |args, _kwargs| {
                Python::with_gil(|py| {
                    let map = PYClassLookupMap::from_app_data(app_data);
                    let slf = args.get_item(0)?.into_py(py);
                    let model_object_wrapper: ModelObjectWrapper = slf.getattr(py, "__teo_object__")?.extract(py)?;
                    let value: Value = model_object_wrapper.object.get_value(field_name).unwrap();
                    Ok::<PyObject, PyErr>(teo_value_to_py_any(py, &value, map)?)    
                })
            })?;
            let field_property_wrapped = property_wrapper.call1((field_property_getter,))?;
            let field_property_setter = PyCFunction::new_closure_bound(py, Some(snake_case_field_name_c), None, move |args, _kwargs| {
                Python::with_gil(|py| {
                    let slf = args.get_item(0)?;
                    let value = args.get_item(1)?;
                    let value = py_any_to_teo_value(py, &value)?;
                    let model_object_wrapper: ModelObjectWrapper = slf.getattr("__teo_object__")?.extract()?;
                    model_object_wrapper.object.set_value(field_name, value).unwrap();
                    Ok::<(), PyErr>(())
                })
            })?;
            let field_property_wrapped = field_property_wrapped.call_method1("setter", (field_property_setter,))?;
            model_object_class.setattr(py, snake_case_field_name, field_property_wrapped)?;
        }
        // relations
        for relation in model.relations().values() {
            let base_relation_name = Box::leak(Box::new(relation.name().to_snake_case())).as_str();
            let base_relation_name_c = Box::leak(Box::new(CString::new(base_relation_name)?)).as_c_str();
            if relation.is_vec() {
                // to many
                // get
                let get = PyCFunction::new_closure_bound(py, Some(base_relation_name_c), None, move |args, _kwargs| {
                    Python::with_gil(|py| {
                        let slf = args.get_item(0)?.into_py(py);
                        let model_object_wrapper: ModelObjectWrapper = slf.getattr(py, "__teo_object__")?.extract(py)?;
                        let find_many_arg = if args.len()? > 1 {
                            let py_dict = args.get_item(1)?;
                            check_py_dict(&py_dict)?;
                            py_any_to_teo_value(py, &py_dict)?
                        } else {
                            Value::Dictionary(IndexMap::new())
                        };
                        let coroutine = pyo3_async_runtimes::tokio::future_into_py::<_, PyObject>(py, (|| async move {
                            let value: Vec<model::Object> = model_object_wrapper.object.force_get_relation_objects(relation.name(), &find_many_arg).await?;
                            Python::with_gil(|py| {
                                let map = PYClassLookupMap::from_app_data(app_data);
                                let list = PyList::empty_bound(py);
                                for v in value {
                                    list.append(teo_model_object_to_py_any(py, &v, map)?)?;
                                }
                                Ok::<PyObject, PyErr>(list.into_py(py))    
                            })
                        })())?;
                        Ok::<PyObject, PyErr>(coroutine.into_py(py))
                    })
                })?;
                teo_wrap_builtin.call1((model_object_class.bind(py), base_relation_name, get))?;
                // set
                let set_name = Box::leak(Box::new("set_".to_owned() + base_relation_name)).as_str();
                let set_name_c = Box::leak(Box::new(CString::new(set_name)?)).as_c_str();
                let set = PyCFunction::new_closure_bound(py, Some(set_name_c), None, move |args, _kwargs| {
                    Python::with_gil(|py| {
                        let slf = args.get_item(0)?.into_py(py);
                        let model_object_wrapper: ModelObjectWrapper = slf.getattr(py, "__teo_object__")?.extract(py)?;
                        let mut objects = vec![];
                        if args.len()? > 1 {
                            let binding = args.get_item(1)?;
                            let py_list: &Bound<PyList> = binding.downcast()?;
                            for item in py_list {
                                objects.push(teo_model_object_from_py_model_object(py, item.into_py(py))?);
                            }
                        }
                        let coroutine = pyo3_async_runtimes::tokio::future_into_py::<_, PyObject>(py, (|| async move {
                            model_object_wrapper.object.force_set_relation_objects(relation.name(), objects).await;
                            Python::with_gil(|py| {
                                Ok::<PyObject, PyErr>(().into_py(py))    
                            })
                        })())?;
                        Ok::<PyObject, PyErr>(coroutine.into_py(py))
                    })
                })?;
                teo_wrap_builtin.call1((model_object_class.bind(py), set_name, set))?;
                // add
                let add_to_name = Box::leak(Box::new("add_to_".to_owned() + base_relation_name)).as_str();
                let add_to_name_c = Box::leak(Box::new(CString::new(add_to_name)?)).as_c_str();
                let add_to = PyCFunction::new_closure_bound(py, Some(add_to_name_c), None, move |args, _kwargs| {
                    Python::with_gil(|py| {
                        let slf = args.get_item(0)?.into_py(py);
                        let model_object_wrapper: ModelObjectWrapper = slf.getattr(py, "__teo_object__")?.extract(py)?;
                        let mut objects = vec![];
                        if args.len()? > 1 {
                            let binding = args.get_item(1)?;
                            let py_list: &Bound<PyList> = binding.downcast()?;
                            for item in py_list {
                                objects.push(teo_model_object_from_py_model_object(py, item.into_py(py))?);
                            }
                        }
                        let coroutine = pyo3_async_runtimes::tokio::future_into_py::<_, PyObject>(py, (|| async move {
                            model_object_wrapper.object.force_add_relation_objects(relation.name(), objects).await;
                            Python::with_gil(|py| {
                                Ok::<PyObject, PyErr>(().into_py(py))    
                            })
                        })())?;
                        Ok::<PyObject, PyErr>(coroutine.into_py(py))
                    })
                })?;
                teo_wrap_builtin.call1((model_object_class.bind(py), add_to_name, add_to))?;
                // remove
                let remove_from_name = Box::leak(Box::new("remove_from_".to_owned() + base_relation_name)).as_str();
                let remove_from_name_c = Box::leak(Box::new(CString::new(remove_from_name)?)).as_c_str();
                let remove_from = PyCFunction::new_closure_bound(py, Some(remove_from_name_c), None, move |args, _kwargs| {
                    Python::with_gil(|py| {
                        let slf = args.get_item(0)?.into_py(py);
                        let model_object_wrapper: ModelObjectWrapper = slf.getattr(py, "__teo_object__")?.extract(py)?;
                        let mut objects = vec![];
                        if args.len()? > 1 {
                            let binding = args.get_item(1)?;
                            let py_list: &Bound<PyList> = binding.downcast()?;
                            for item in py_list {
                                objects.push(teo_model_object_from_py_model_object(py, item.into_py(py))?);
                            }
                        }
                        let coroutine = pyo3_async_runtimes::tokio::future_into_py::<_, PyObject>(py, (|| async move {
                            model_object_wrapper.object.force_remove_relation_objects(relation.name(), objects).await;
                            Python::with_gil(|py| {
                                Ok::<PyObject, PyErr>(().into_py(py))    
                            })
                        })())?;
                        Ok::<PyObject, PyErr>(coroutine.into_py(py))
                    })
                })?;
                teo_wrap_builtin.call1((model_object_class.bind(py), remove_from_name, remove_from))?;
            } else {
                // to single
                // get
                let get = PyCFunction::new_closure_bound(py, Some(base_relation_name_c), None, move |args, _kwargs| {
                    Python::with_gil(|py| {
                        let map = PYClassLookupMap::from_app_data(app_data);
                        let slf = args.get_item(0)?.into_py(py);
                        let model_object_wrapper: ModelObjectWrapper = slf.getattr(py, "__teo_object__")?.extract(py)?;
                        let coroutine = pyo3_async_runtimes::tokio::future_into_py::<_, PyObject>(py, (|| async move {
                            let value: Option<model::Object> = model_object_wrapper.object.force_get_relation_object(relation.name()).await?;
                            Python::with_gil(|py| {
                                match value {
                                    Some(value) => Ok::<PyObject, PyErr>(teo_model_object_to_py_any(py, &value, map)?.into_py(py)),
                                    None => Ok::<PyObject, PyErr>(().into_py(py))  
                                }
                            })
                        })())?;
                        Ok::<PyObject, PyErr>(coroutine.into_py(py))
                    })
                })?;
                teo_wrap_builtin.call1((model_object_class.bind(py), base_relation_name, get))?;
                // set
                let set_name = Box::leak(Box::new("set_".to_owned() + base_relation_name)).as_str();
                let set_name_c = Box::leak(Box::new(CString::new(set_name)?)).as_c_str();
                let set = PyCFunction::new_closure_bound(py, Some(set_name_c), None, move |args, _kwargs| {
                    Python::with_gil(|py| {
                        let slf = args.get_item(0)?.into_py(py);
                        let model_object_wrapper: ModelObjectWrapper = slf.getattr(py, "__teo_object__")?.extract(py)?;
                        let next_argument = if args.len()? > 1 {
                            let arg1 = args.get_item(1)?;
                            let teo_object = py_any_to_teo_value(py, &arg1)?;
                            if let Some(model_object) = teo_object.as_model_object()  {
                                Some(model_object.clone())
                            } else {
                                None
                            }
                        } else {
                            None
                        };
                        let coroutine = pyo3_async_runtimes::tokio::future_into_py::<_, PyObject>(py, (|| async move {
                            model_object_wrapper.object.force_set_relation_object(relation.name(), next_argument).await;
                            Python::with_gil(|py| {
                                Ok::<PyObject, PyErr>(().into_py(py))    
                            })
                        })())?;
                        Ok::<PyObject, PyErr>(coroutine.into_py(py))
                    })
                })?;
                teo_wrap_builtin.call1((model_object_class.bind(py), set_name, set))?;
            }
        }
        // properties
        for model_property in model.properties().values() {
            let field_name: &str = Box::leak(Box::new(model_property.name().to_string())).as_str();
            if model_property.setter().is_some() {
                let name: &str = Box::leak(Box::new("set_".to_owned() + &field_name.to_snake_case())).as_str();
                let name_c = Box::leak(Box::new(CString::new(name)?)).as_c_str();
                let setter = PyCFunction::new_closure_bound(py, Some(name_c), None, move |args, _kwargs| {
                    Python::with_gil(|py| {
                        let slf = args.get_item(0)?;
                        let value = args.get_item(1)?;
                        let value = py_any_to_teo_value(py, &value)?;
                        let model_object_wrapper: ModelObjectWrapper = slf.getattr("__teo_object__")?.extract()?;
                        let coroutine = pyo3_async_runtimes::tokio::future_into_py::<_, PyObject>(py, (|| async move {
                            let result = model_object_wrapper.object.set_property(field_name, value).await?;
                            Python::with_gil(|py| {
                                Ok(result.into_py(py))
                            })
                        })())?;
                        Ok::<PyObject, PyErr>(coroutine.into_py(py))
                    })
                })?;
                teo_wrap_builtin.call1((model_object_class.bind(py), name, setter))?;
            }
            if model_property.getter().is_some() {
                let name: &str = Box::leak(Box::new(field_name.to_snake_case())).as_str();
                let name_c = Box::leak(Box::new(CString::new(name)?)).as_c_str();
                let getter = PyCFunction::new_closure_bound(py, Some(name_c), None, move |args, _kwargs| {
                    Python::with_gil(|py| {
                        let map = PYClassLookupMap::from_app_data(app_data);
                        let slf = args.get_item(0)?.into_py(py);
                        let model_object_wrapper: ModelObjectWrapper = slf.getattr(py, "__teo_object__")?.extract(py)?;
                        let coroutine = pyo3_async_runtimes::tokio::future_into_py::<_, PyObject>(py, (|| async move {
                            let result = model_object_wrapper.object.get_property_value(field_name).await?;
                            Python::with_gil(|py| {
                                let any = teo_value_to_py_any(py, &result, map)?;
                                Ok(any.into_py(py))
                            })
                        })())?;
                        Ok::<PyObject, PyErr>(coroutine.into_py(py))
                    })
                })?;
                teo_wrap_builtin.call1((model_object_class.bind(py), name, getter))?;
            }
        }
    }
    for namespace in namespace.namespaces().values() {
        let namespace_name = Box::leak(Box::new(namespace.path().join("."))).as_str();
        let namespace_name_c = Box::leak(Box::new(CString::new(namespace_name)?)).as_c_str();
        let namespace_property = PyCFunction::new_closure_bound(py, Some(namespace_name_c), None, move |args, _kwargs| {
            let next_ctx_object = Python::with_gil(|py| {
                let map = PYClassLookupMap::from_app_data(app_data);
                let slf = args.get_item(0)?;
                let transaction_ctx_wrapper: TransactionCtxWrapper = slf.getattr("__teo_transaction_ctx__")?.extract()?;
                let next_ctx_class = map.ctx(&namespace_name)?.unwrap();
                let next_ctx_object = next_ctx_class.call_method1(py, "__new__", (&next_ctx_class,))?;
                next_ctx_object.setattr(py, "__teo_transaction_ctx__", transaction_ctx_wrapper.clone())?;
                Ok::<PyObject, PyErr>(next_ctx_object)
            })?;
            Ok::<PyObject, PyErr>(next_ctx_object)
        })?;
        let namespace_property_wrapped = property_wrapper.call1((namespace_property,))?;
        ctx_class.setattr(py, namespace.path().last().unwrap().to_snake_case().as_str(), namespace_property_wrapped)?;
    }
    // transaction
    let transaction = PyCFunction::new_closure_bound(py, Some(c"transaction"), Some(c"Run transaction."), move |args, _kwargs| {
        Python::with_gil(|py| {
            let slf = args.get_item(0)?;
            let transaction_ctx_wrapper: TransactionCtxWrapper = slf.getattr("__teo_transaction_ctx__")?.extract()?;
            let argument = args.get_item(1)?.into_py(py);
            let shared_argument = &*Box::leak(Box::new(argument));
            let coroutine = pyo3_async_runtimes::tokio::future_into_py::<_, PyObject>(py, (move || async move {
                let retval: pyo3::prelude::Py<PyAny> = transaction_ctx_wrapper.ctx.run_transaction(move |ctx: transaction::Ctx| async move {
                    let user_retval = Python::with_gil(move |py| {
                        let map = PYClassLookupMap::from_app_data(app_data);
                        let ctx_python = map.teo_transaction_ctx_to_py_ctx_object(py, ctx, "")?;
                        let coroutine_or_value = shared_argument.call1(py, (ctx_python,))?;
                        let thread_locals = pyo3_async_runtimes::tokio::get_current_locals(py)?;
                        Ok::<(PyObject, TaskLocals), teo::prelude::Error>((coroutine_or_value, thread_locals))
                    })?;
                    Ok(await_coroutine_if_needed_value_with_locals(&user_retval.0, &user_retval.1).await?)
                }).await?;
                Python::with_gil(|py| {
                    Ok::<PyObject, PyErr>(retval.into_py(py))    
                })
            })())?;
            Ok::<PyObject, PyErr>(coroutine.into_py(py))
        })
    })?;
    teo_wrap_builtin.call1((&ctx_class, "transaction", transaction))?;
    // finish
    ctx_class.setattr(py, "__teo_initialized__", true)?;
    Ok(())
}


fn find_unique_object_function<'py>(py: Python<'py>, app_data: &'static AppData) -> PyResult<Bound<'py, PyCFunction>> {
    Ok(PyCFunction::new_closure_bound(py, Some(c"find_unique_object"), Some(c"Find a unique object."), move |args, _kwargs| {
        Python::with_gil(|py| {
            let slf = args.get_item(0)?.into_py(py);
            let model_ctx_wrapper: ModelCtxWrapper = slf.getattr(py, "__teo_model_ctx__")?.extract(py)?;
            let find_many_arg = if args.len()? > 1 {
                let py_dict = args.get_item(1)?;
                check_py_dict(&py_dict)?;
                py_any_to_teo_value(py, &py_dict)?
            } else {
                Value::Dictionary(IndexMap::new())
            };
            let coroutine = pyo3_async_runtimes::tokio::future_into_py(py, (|| async move {
                let result: Option<model::Object> = model_ctx_wrapper.ctx.find_unique(&find_many_arg).await?;
                Python::with_gil(|py| {
                    match result {
                        Some(object) => {
                            let map = PYClassLookupMap::from_app_data(app_data);
                            map.teo_model_object_to_py_model_object_object(py, object)
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

fn find_first_object_function<'py>(py: Python<'py>, app_data: &'static AppData) -> PyResult<Bound<'py, PyCFunction>> {
    Ok(PyCFunction::new_closure_bound(py, Some(c"find_first_object"), Some(c"Find an object."), move |args, _kwargs| {
        Python::with_gil(|py| {
            let slf = args.get_item(0)?.into_py(py);
            let model_ctx_wrapper: ModelCtxWrapper = slf.getattr(py, "__teo_model_ctx__")?.extract(py)?;
            let find_many_arg = if args.len()? > 1 {
                let py_dict = args.get_item(1)?;
                check_py_dict(&py_dict)?;
                py_any_to_teo_value(py, &py_dict)?
            } else {
                Value::Dictionary(IndexMap::new())
            };
            let coroutine = pyo3_async_runtimes::tokio::future_into_py(py, (|| async move {
                let result: Option<model::Object> = model_ctx_wrapper.ctx.find_first(&find_many_arg).await?;
                Python::with_gil(|py| {
                    match result {
                        Some(object) => {
                            let map = PYClassLookupMap::from_app_data(app_data);
                            map.teo_model_object_to_py_model_object_object(py, object)
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

fn find_many_objects_function<'py>(py: Python<'py>, app_data: &'static AppData) -> PyResult<Bound<'py, PyCFunction>> {
    Ok(PyCFunction::new_closure_bound(py, Some(c"find_many_objects"), Some(c"Find many objects."), move |args, _kwargs| {
        Python::with_gil(|py| {
            let slf = args.get_item(0)?.into_py(py);
            let model_ctx_wrapper: ModelCtxWrapper = slf.getattr(py, "__teo_model_ctx__")?.extract(py)?;
            let find_many_arg = if args.len()? > 1 {
                let py_dict = args.get_item(1)?;
                check_py_dict(&py_dict)?;
                py_any_to_teo_value(py, &py_dict)?
            } else {
                Value::Dictionary(IndexMap::new())
            };
            let coroutine = pyo3_async_runtimes::tokio::future_into_py::<_, PyObject>(py, (|| async move {
                let result: Vec<model::Object> = model_ctx_wrapper.ctx.find_many(&find_many_arg).await?;
                Python::with_gil(|py| {
                    let py_result = PyList::empty_bound(py);
                    let map = PYClassLookupMap::from_app_data(app_data);
                    for object in result {
                        let instance = map.teo_model_object_to_py_model_object_object(py, object)?;
                        py_result.append(instance)?;
                    }
                    Ok(py_result.into_py(py))
                })
            })())?;
            Ok::<PyObject, PyErr>(coroutine.into_py(py))
        })
    })?)
}

fn create_object_function<'py>(py: Python<'py>, app_data: &'static AppData) -> PyResult<Bound<'py, PyCFunction>> {
    Ok(PyCFunction::new_closure_bound(py, Some(c"create_object"), Some(c"Create a new object."), move |args, _kwargs| {
        Python::with_gil(|py| {
            let slf = args.get_item(0)?.into_py(py);
            let model_ctx_wrapper: ModelCtxWrapper = slf.getattr(py, "__teo_model_ctx__")?.extract(py)?;
            let create_arg = if args.len()? > 1 {
                let py_dict = args.get_item(1)?;
                check_py_dict(&py_dict)?;
                py_any_to_teo_value(py, &py_dict)?
            } else {
                Value::Dictionary(IndexMap::new())
            };
            let coroutine = pyo3_async_runtimes::tokio::future_into_py::<_, PyObject>(py, (|| async move {
                let result: model::Object = model_ctx_wrapper.ctx.create_object(&create_arg).await?;
                Python::with_gil(|py| {
                    let map = PYClassLookupMap::from_app_data(app_data);
                    let instance = map.teo_model_object_to_py_model_object_object(py, result)?;
                    Ok(instance.into_py(py))
                })
            })())?;
            Ok::<PyObject, PyErr>(coroutine.into_py(py))
        })
    })?)
}

fn count_objects_function<'py>(py: Python<'py>) -> PyResult<Bound<PyCFunction>> {
    Ok(PyCFunction::new_closure_bound(py, Some(c"count_objects"), Some(c"Count records."), move |args, _kwargs| {
        Python::with_gil(|py| {
            let slf = args.get_item(0)?.into_py(py);
            let model_ctx_wrapper: ModelCtxWrapper = slf.getattr(py, "__teo_model_ctx__")?.extract(py)?;
            let count_arg = if args.len()? > 1 {
                let py_dict = args.get_item(1)?;
                check_py_dict(&py_dict)?;
                py_any_to_teo_value(py, &py_dict)?
            } else {
                Value::Dictionary(IndexMap::new())
            };
            let coroutine = pyo3_async_runtimes::tokio::future_into_py::<_, PyObject>(py, (|| async move {
                let result: usize = model_ctx_wrapper.ctx.count_objects(&count_arg).await?;
                Python::with_gil(|py| {
                    Ok(result.into_py(py))
                })
            })())?;
            Ok::<PyObject, PyErr>(coroutine.into_py(py))
        })
    })?)
}

fn count_fields_function<'py>(py: Python<'py>, app_data: &'static AppData) -> PyResult<Bound<'py, PyCFunction>> {
    Ok(PyCFunction::new_closure_bound(py, Some(c"count_fields"), Some(c"Count records."), move |args, _kwargs| {
        Python::with_gil(|py| {
            let slf = args.get_item(0)?.into_py(py);
            let model_ctx_wrapper: ModelCtxWrapper = slf.getattr(py, "__teo_model_ctx__")?.extract(py)?;
            let count_arg = if args.len()? > 1 {
                let py_dict = args.get_item(1)?;
                check_py_dict(&py_dict)?;
                py_any_to_teo_value(py, &py_dict)?
            } else {
                Value::Dictionary(IndexMap::new())
            };
            let coroutine = pyo3_async_runtimes::tokio::future_into_py::<_, PyObject>(py, (|| async move {
                let map = PYClassLookupMap::from_app_data(app_data);
                let result: Value = model_ctx_wrapper.ctx.count_fields(&count_arg).await?;
                Python::with_gil(|py| {
                    teo_value_to_py_any(py, &result, map)
                })
            })())?;
            Ok::<PyObject, PyErr>(coroutine.into_py(py))
        })
    })?)
}

fn aggregate_function<'py>(py: Python<'py>, app_data: &'static AppData) -> PyResult<Bound<'py, PyCFunction>> {
    Ok(PyCFunction::new_closure_bound(py, Some(c"aggregate"), Some(c"Aggregate on records."), move |args, _kwargs| {
        Python::with_gil(|py| {
            let slf = args.get_item(0)?.into_py(py);
            let model_ctx_wrapper: ModelCtxWrapper = slf.getattr(py, "__teo_model_ctx__")?.extract(py)?;
            let aggregate_arg = if args.len()? > 1 {
                let py_dict = args.get_item(1)?;
                check_py_dict(&py_dict)?;
                py_any_to_teo_value(py, &py_dict)?
            } else {
                Value::Dictionary(IndexMap::new())
            };
            let coroutine = pyo3_async_runtimes::tokio::future_into_py::<_, PyObject>(py, (|| async move {
                let map = PYClassLookupMap::from_app_data(app_data);
                let result: Value = model_ctx_wrapper.ctx.aggregate(&aggregate_arg).await?;
                Python::with_gil(|py| {
                    teo_value_to_py_any(py, &result, map)
                })
            })())?;
            Ok::<PyObject, PyErr>(coroutine.into_py(py))
        })
    })?)
}

fn group_by_function<'py>(py: Python<'py>, app_data: &'static AppData) -> PyResult<Bound<'py, PyCFunction>> {
    Ok(PyCFunction::new_closure_bound(py, Some(c"group_by"), Some(c"Group by on records."), move |args, _kwargs| {
        Python::with_gil(|py| {
            let slf = args.get_item(0)?.into_py(py);
            let model_ctx_wrapper: ModelCtxWrapper = slf.getattr(py, "__teo_model_ctx__")?.extract(py)?;
            let group_by_arg = if args.len()? > 1 {
                let py_dict = args.get_item(1)?;
                check_py_dict(&py_dict)?;
                py_any_to_teo_value(py, &py_dict)?
            } else {
                Value::Dictionary(IndexMap::new())
            };
            let coroutine = pyo3_async_runtimes::tokio::future_into_py::<_, PyObject>(py, (|| async move {
                let result: Vec<Value> = model_ctx_wrapper.ctx.group_by(&group_by_arg).await?;
                Python::with_gil(|py| {
                    let map = PYClassLookupMap::from_app_data(app_data);
                    let py_result = PyList::empty_bound(py);
                    for value in result {
                        let instance = teo_value_to_py_any(py, &value, map)?;
                        py_result.append(instance)?;
                    }
                    Ok(py_result.into_py(py))
                })
            })())?;
            Ok::<PyObject, PyErr>(coroutine.into_py(py))
        })
    })?)
}

fn sql_function<'py>(py: Python<'py>, app_data: &'static AppData) -> PyResult<Bound<'py, PyCFunction>> {
    Ok(PyCFunction::new_closure_bound(py, Some(c"sql"), Some(c"Run custom SQL clause."), move |args, _kwargs| {
        Python::with_gil(|py| {
            let slf = args.get_item(0)?.into_py(py);
            let model_ctx_wrapper: ModelCtxWrapper = slf.getattr(py, "__teo_model_ctx__")?.extract(py)?;
            let sql_string_any: Bound<PyAny> = args.get_item(1)?;
            let sql_string: String = sql_string_any.extract()?;
            let coroutine = pyo3_async_runtimes::tokio::future_into_py::<_, PyObject>(py, (|| async move {
                let result: Vec<Value> = model_ctx_wrapper.ctx.sql(&sql_string).await?;
                Python::with_gil(|py| {
                    let py_result = PyList::empty_bound(py);
                    let map = PYClassLookupMap::from_app_data(app_data);
                    for value in result {
                        let instance = teo_value_to_py_any(py, &value, map)?;
                        py_result.append(instance)?;
                    }
                    Ok(py_result.into_py(py))
                })
            })())?;
            Ok::<PyObject, PyErr>(coroutine.into_py(py))
        })
    })?)
}

fn is_new_function<'py>(py: Python<'py>) -> PyResult<Bound<PyCFunction>> {
    Ok(PyCFunction::new_closure_bound(py, Some(c"is_new"), Some(c"Whether this model object is new."), move |args, _kwargs| {
        Python::with_gil(|py| {
            let slf = args.get_item(0)?.into_py(py);
            let model_object_wrapper: ModelObjectWrapper = slf.getattr(py, "__teo_object__")?.extract(py)?;
            Ok::<PyObject, PyErr>(model_object_wrapper.object.is_new().into_py(py))
        })
    })?)
}

fn is_modified_function<'py>(py: Python<'py>) -> PyResult<Bound<PyCFunction>> {
    Ok(PyCFunction::new_closure_bound(py, Some(c"is_modified"), Some(c"Whether this model object is modified."), move |args, _kwargs| {
        Python::with_gil(|py| {
            let slf = args.get_item(0)?.into_py(py);
            let model_object_wrapper: ModelObjectWrapper = slf.getattr(py, "__teo_object__")?.extract(py)?;
            Ok::<PyObject, PyErr>(model_object_wrapper.object.is_modified().into_py(py))
        })
    })?)
}

fn set_function<'py>(py: Python<'py>) -> PyResult<Bound<PyCFunction>> {
    Ok(PyCFunction::new_closure_bound(py, Some(c"set"), Some(c"Set values to this object."), move |args, _kwargs| {
        Python::with_gil(|py| {
            let slf = args.get_item(0)?.into_py(py);
            let model_object_wrapper: ModelObjectWrapper = slf.getattr(py, "__teo_object__")?.extract(py)?;
            let set_arg = if args.len()? > 1 {
                let py_dict = args.get_item(1)?;
                check_py_dict(&py_dict)?;
                py_any_to_teo_value(py, &py_dict)?
            } else {
                Value::Dictionary(IndexMap::new())
            };
            let coroutine = pyo3_async_runtimes::tokio::future_into_py::<_, PyObject>(py, (|| async move {
                let result: () = model_object_wrapper.object.set_teon(&set_arg).await?;
                Python::with_gil(|py| {
                    Ok(result.into_py(py))
                })
            })())?;
            Ok::<PyObject, PyErr>(coroutine.into_py(py))
        })
    })?)
}

fn update_function<'py>(py: Python<'py>) -> PyResult<Bound<PyCFunction>> {
    Ok(PyCFunction::new_closure_bound(py, Some(c"update"), Some(c"Update values on this object."), move |args, _kwargs| {
        Python::with_gil(|py| {
            let slf = args.get_item(0)?.into_py(py);
            let model_object_wrapper: ModelObjectWrapper = slf.getattr(py, "__teo_object__")?.extract(py)?;
            let set_arg = if args.len()? > 1 {
                let py_dict = args.get_item(1)?;
                check_py_dict(&py_dict)?;
                py_any_to_teo_value(py, &py_dict)?
            } else {
                Value::Dictionary(IndexMap::new())
            };
            let coroutine = pyo3_async_runtimes::tokio::future_into_py::<_, PyObject>(py, (|| async move {
                let result: () = model_object_wrapper.object.update_teon(&set_arg).await?;
                Python::with_gil(|py| {
                    Ok(result.into_py(py))
                })
            })())?;
            Ok::<PyObject, PyErr>(coroutine.into_py(py))
        })
    })?)
}

fn save_function<'py>(py: Python<'py>) -> PyResult<Bound<PyCFunction>> {
    Ok(PyCFunction::new_closure_bound(py, Some(c"save"), Some(c"Save this object."), move |args, _kwargs| {
        Python::with_gil(|py| {
            let slf = args.get_item(0)?.into_py(py);
            let model_object_wrapper: ModelObjectWrapper = slf.getattr(py, "__teo_object__")?.extract(py)?;
            let coroutine = pyo3_async_runtimes::tokio::future_into_py::<_, PyObject>(py, (|| async move {
                let result: () = model_object_wrapper.object.save().await?;
                Python::with_gil(|py| {
                    Ok(result.into_py(py))
                })
            })())?;
            Ok::<PyObject, PyErr>(coroutine.into_py(py))
        })
    })?)
}

fn delete_function<'py>(py: Python<'py>) -> PyResult<Bound<PyCFunction>> {
    Ok(PyCFunction::new_closure_bound(py, Some(c"delete"), Some(c"Delete this object."), move |args, _kwargs| {
        Python::with_gil(|py| {
            let slf = args.get_item(0)?.into_py(py);
            let model_object_wrapper: ModelObjectWrapper = slf.getattr(py, "__teo_object__")?.extract(py)?;
            let coroutine = pyo3_async_runtimes::tokio::future_into_py::<_, PyObject>(py, (|| async move {
                let result: () = model_object_wrapper.object.delete().await?;
                Python::with_gil(|py| {
                    Ok(result.into_py(py))
                })
            })())?;
            Ok::<PyObject, PyErr>(coroutine.into_py(py))
        })
    })?)
}

fn to_teon_function<'py>(py: Python<'py>, app_data: &'static AppData) -> PyResult<Bound<'py, PyCFunction>> {
    Ok(PyCFunction::new_closure_bound(py, Some(c"to_teon"), Some(c"Convert this object to a Teon object."), move |args, _kwargs| {
        Python::with_gil(|py| {
            let slf = args.get_item(0)?.into_py(py);
            let model_object_wrapper: ModelObjectWrapper = slf.getattr(py, "__teo_object__")?.extract(py)?;
            let coroutine = pyo3_async_runtimes::tokio::future_into_py::<_, PyObject>(py, (|| async move {
                let result: Value = model_object_wrapper.object.to_teon().await?;
                let map = PYClassLookupMap::from_app_data(app_data);
                Python::with_gil(|py| {
                    teo_value_to_py_any(py, &result, map)
                })
            })())?;
            Ok::<PyObject, PyErr>(coroutine.into_py(py))
        })
    })?)
}

fn repr_function<'py>(py: Python<'py>, app_data: &'static AppData) -> PyResult<Bound<'py, PyCFunction>> {
    Ok(PyCFunction::new_closure_bound(py, Some(c"__repr__"), None, move |args, _kwargs| {
        Python::with_gil(|py| {
            let slf = args.get_item(0)?.into_py(py);
            let model_object_wrapper: ModelObjectWrapper = slf.getattr(py, "__teo_object__")?.extract(py)?;
            let result = PyDict::new_bound(py);
            let value_map = model_object_wrapper.object.inner.value_map.lock().unwrap();
            let map = PYClassLookupMap::from_app_data(app_data);
            for (k, v) in value_map.iter() {
                result.set_item(k, teo_value_to_py_any(py, v, map)?)?;
            }
            let dict_repr = result.call_method("__repr__", (), None)?;
            let dict_repr_str: &str = dict_repr.extract()?;
            let prefix = format!("{}(", model_object_wrapper.object.model().path().join("."));
            let suffix = ")";
            Ok::<PyObject, PyErr>(format!("{}{}{}", prefix, dict_repr_str, suffix).into_py(py))
        })
    })?)
}
