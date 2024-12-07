use std::sync::Arc;
use indexmap::IndexMap;
use inflector::Inflector;
use pyo3_async_runtimes::TaskLocals;
use teo::prelude::app::data::AppData;
use teo::prelude::traits::named::Named;
use ::teo::prelude::App;
use pyo3::{Bound, IntoPyObjectExt, PyAny, PyErr, PyObject, PyResult, Python};
use pyo3::types::{PyAnyMethods, PyCFunction, PyDict, PyList, PyListMethods, PyNone};
use teo::prelude::{Namespace, Value, model, transaction};
use crate::object::array::teo_array_to_py_any;
use crate::object::model::teo_model_object_to_py_any;
use crate::object::value::{teo_value_to_py_any, py_any_to_teo_value};
use crate::utils::await_coroutine_if_needed::await_coroutine_if_needed_value_with_locals;
use crate::utils::check_py_dict::check_py_dict;
use crate::utils::cstr::static_cstr;
use super::wrappers::{ModelCtxWrapper, TransactionCtxWrapper, ModelObjectWrapper};
use super::{CreateDynamicClasses, DynamicClasses, DynamicClassesBuilder, FetchDynamicClasses, QueryDynamicClasses};

pub(crate) fn teo_model_object_from_py_model_object(py: Python<'_>, model_object: PyObject) -> PyResult<model::Object> {
    let wrapper: ModelObjectWrapper = model_object.getattr(py, "__teo_object__")?.extract(py)?;
    Ok(wrapper.object.clone())
}

pub fn synthesize_dynamic_python_classes(app: &App, py: Python<'_>) -> PyResult<()> {
    let mut builder = DynamicClassesBuilder::new();
    synthesize_dynamic_python_classes_for_namespace(&mut builder, app, app.compiled_main_namespace(), py)?;
    app.app_data().set_dynamic_classes(Arc::new(builder.build()))?;
    Ok(())
}

fn synthesize_dynamic_python_classes_for_namespace(builder: &mut DynamicClassesBuilder, app: &App, namespace: &Namespace, py: Python<'_>) -> PyResult<()> {
    synthesize_direct_dynamic_python_classes_for_namespace(builder, app, namespace, py)?;
    for namespace in namespace.namespaces().values() {
        synthesize_dynamic_python_classes_for_namespace(builder, app, namespace, py)?;
    }
    Ok(())
}

fn synthesize_direct_dynamic_python_classes_for_namespace(builder: &mut DynamicClassesBuilder, app: &App, namespace: &Namespace, py: Python<'_>) -> PyResult<()> {
    let app_data = app.app_data().clone();
    let main = py.import("__main__")?;
    let teo_wrap_builtin = main.getattr("teo_wrap_builtin")?;
    let teo_wrap_async = main.getattr("teo_wrap_async")?;
    let builtins = py.import("builtins")?;
    let property_wrapper = builtins.getattr("property")?;
    let ctx_class = builder.ctx_or_create(&namespace.path().join("."), py)?;
    for model in namespace.models().values() {
        let model_name = model.path().join(".");
        let model_name_cstr = static_cstr(model_name.as_str())?;
        let model_property_name = model.path().last().unwrap().to_snake_case();
        let model_property = PyCFunction::new_closure(py, Some(model_name_cstr), None, {
            let model = model.clone();
            let model_name = model_name.clone();
            let app_data = app_data.clone();
            move |args, _kwargs| {
                let model_class_object = Python::with_gil(|py| {
                    let map = DynamicClasses::retrieve(&app_data)?;
                    let slf = args.get_item(0)?;
                    let transaction_ctx_wrapper: TransactionCtxWrapper = slf.getattr("__teo_transaction_ctx__")?.extract()?;
                    let model_ctx = transaction_ctx_wrapper.ctx.model_ctx_for_model_at_path(&model.path()).unwrap();
                    let model_class_class = map.class(&model_name)?.unwrap();
                    let model_class_object = model_class_class.call_method1(py, "__new__", (model_class_class.clone_ref(py),))?;
                    model_class_object.setattr(py, "__teo_model_ctx__", ModelCtxWrapper::new(model_ctx))?;
                    Ok::<PyObject, PyErr>(model_class_object)
                })?;
                Ok::<PyObject, PyErr>(model_class_object)
            }
        })?;
        let model_property_wrapped = property_wrapper.call1((model_property,))?;
        ctx_class.setattr(py, model_property_name.as_str(), model_property_wrapped)?;
        // class object methods
        let model_class_class = builder.class_or_create(&model_name, py)?.into_bound(py);
        // find unique
        let find_unique_object = find_unique_object_function(py, &app_data)?;
        teo_wrap_builtin.call1((&model_class_class, "find_unique_object", teo_wrap_async.call1((find_unique_object,))?))?;
        // find first
        let find_first_object = find_first_object_function(py, &app_data)?;
        teo_wrap_builtin.call1((&model_class_class, "find_first_object", teo_wrap_async.call1((find_first_object,))?))?;
        // find many
        let find_many_objects = find_many_objects_function(py, &app_data)?;
        teo_wrap_builtin.call1((&model_class_class, "find_many_objects", teo_wrap_async.call1((find_many_objects,))?))?;
        // create
        let create_object: Bound<'_, PyCFunction> = create_object_function(py, &app_data)?;
        teo_wrap_builtin.call1((&model_class_class, "create_object", teo_wrap_async.call1((create_object,))?))?;
        // count objects
        let count_objects = count_objects_function(py)?;
        teo_wrap_builtin.call1((&model_class_class, "count_objects", count_objects))?;
        // count fields
        let count_fields = count_fields_function(py, &app_data)?;
        teo_wrap_builtin.call1((&model_class_class, "count_fields", count_fields))?;
        // aggregate
        let aggregate = aggregate_function(py, &app_data)?;
        teo_wrap_builtin.call1((&model_class_class, "aggregate", aggregate))?;
        // group by
        let group_by = group_by_function(py, &app_data)?;
        teo_wrap_builtin.call1((&model_class_class, "group_by", group_by))?;
        // sql
        if namespace.database().is_some() && namespace.database().unwrap().is_sql() {
            let sql = sql_function(py, &app_data)?;
            teo_wrap_builtin.call1((&model_class_class, "sql", sql))?;    
        }
        // model object methods
        let model_object_class = builder.object_or_create(&model_name, py)?.into_bound(py);
        // is new
        let is_new = is_new_function(py)?;
        teo_wrap_builtin.call1((&model_object_class, "is_new", is_new))?;
        // is modified
        let is_modified = is_modified_function(py)?;
        teo_wrap_builtin.call1((&model_object_class, "is_modified", is_modified))?;
        // set
        let set = set_function(py)?;
        teo_wrap_builtin.call1((&model_object_class, "set", set))?;
        // update
        let update = update_function(py)?;
        teo_wrap_builtin.call1((&model_object_class, "update", update))?;
        // save
        let save = save_function(py)?;
        teo_wrap_builtin.call1((&model_object_class, "save", save))?;
        // delete
        let delete = delete_function(py)?;
        teo_wrap_builtin.call1((&model_object_class, "delete", delete))?;
        // to teon
        let to_teon = to_teon_function(py, &app_data)?;
        teo_wrap_builtin.call1((&model_object_class, "to_teon", to_teon))?;
        // __repr__
        let repr = repr_function(py, &app_data)?;
        teo_wrap_builtin.call1((&model_object_class, "__repr__", repr))?;
        // fields
        for field in model.fields().values() {
            let snake_case_field_name = field.name().to_snake_case();
            let snake_case_field_name_cstr = static_cstr(snake_case_field_name.as_str())?;
            let field_name = field.name().to_string();
            let field_property_getter = PyCFunction::new_closure(py, Some(snake_case_field_name_cstr), None, {
                let field_name = field_name.clone();
                let app_data = app_data.clone();
                move |args, _kwargs| {
                    Python::with_gil(|py| {
                        let map = DynamicClasses::retrieve(&app_data)?;
                        let slf = args.get_item(0)?;
                        let model_object_wrapper: ModelObjectWrapper = slf.getattr("__teo_object__")?.extract()?;
                        let value: Value = model_object_wrapper.object.get_value(field_name.as_str()).unwrap();
                        Ok::<PyObject, PyErr>(teo_value_to_py_any(py, &value, &map)?)
                    })
                }
            })?;
            let field_property_wrapped = property_wrapper.call1((field_property_getter,))?;
            let field_property_setter = PyCFunction::new_closure(py, Some(snake_case_field_name_cstr), None, {
                let field_name = field_name.clone();
                move |args, _kwargs| {
                    Python::with_gil(|py| {
                        let slf = args.get_item(0)?;
                        let value = args.get_item(1)?;
                        let value = py_any_to_teo_value(py, &value)?;
                        let model_object_wrapper: ModelObjectWrapper = slf.getattr("__teo_object__")?.extract()?;
                        model_object_wrapper.object.set_value(field_name.as_str(), value).unwrap();
                        Ok::<(), PyErr>(())
                    })
                }
            })?;
            let field_property_wrapped = field_property_wrapped.call_method1("setter", (field_property_setter,))?;
            model_object_class.setattr(snake_case_field_name, field_property_wrapped)?;
        }
        // relations
        for relation in model.relations().values() {
            let relation_name = relation.name().to_owned();
            let base_relation_name = relation.name().to_snake_case();
            let base_relation_name_cstr = static_cstr(&base_relation_name)?;
            if relation.is_vec() {
                // to many
                // get
                let get = PyCFunction::new_closure(py, Some(base_relation_name_cstr), None, {
                    let app_data = app_data.clone();
                    let relation_name = relation_name.clone();
                    move |args, _kwargs| {
                        let app_data = app_data.clone();
                        let relation_name = relation_name.clone();
                        Python::with_gil(|py| {
                            let slf = args.get_item(0)?;
                            let model_object_wrapper: ModelObjectWrapper = slf.getattr("__teo_object__")?.extract()?;
                            let find_many_arg = if args.len()? > 1 {
                                let py_dict = args.get_item(1)?;
                                check_py_dict(&py_dict)?;
                                py_any_to_teo_value(py, &py_dict)?
                            } else {
                                Value::Dictionary(IndexMap::new())
                            };
                            let coroutine = pyo3_async_runtimes::tokio::future_into_py::<_, PyObject>(py, (move || {
                                let app_data = app_data.clone();
                                async move {
                                    let value: Vec<model::Object> = model_object_wrapper.object.force_get_relation_objects(&relation_name, &find_many_arg).await?;
                                    Python::with_gil(|py| {
                                        let map = DynamicClasses::retrieve(&app_data)?;
                                        let list = PyList::empty(py);
                                        for v in value {
                                            list.append(teo_model_object_to_py_any(py, &v, &map)?)?;
                                        }
                                        Ok::<PyObject, PyErr>(list.into_any().unbind())    
                                    })    
                                }
                            })())?;
                            Ok::<PyObject, PyErr>(coroutine.unbind())
                        })
                    }
                })?;
                teo_wrap_builtin.call1((&model_object_class, &base_relation_name, get))?;
                // set
                let set_name = "set_".to_owned() + &base_relation_name;
                let set_name_cstr = static_cstr(&set_name)?;
                let set = PyCFunction::new_closure(py, Some(set_name_cstr), None, {
                    let relation_name = relation_name.clone();
                    move |args, _kwargs| {
                        let relation_name = relation_name.clone();
                        Python::with_gil(|py| {
                            let slf = args.get_item(0)?;
                            let model_object_wrapper: ModelObjectWrapper = slf.getattr("__teo_object__")?.extract()?;
                            let mut objects = vec![];
                            if args.len()? > 1 {
                                let binding = args.get_item(1)?;
                                let py_list: &Bound<PyList> = binding.downcast()?;
                                for item in py_list {
                                    objects.push(teo_model_object_from_py_model_object(py, item.unbind())?);
                                }
                            }
                            let coroutine = pyo3_async_runtimes::tokio::future_into_py::<_, PyObject>(py, (|| async move {
                                model_object_wrapper.object.force_set_relation_objects(&relation_name, objects).await;
                                Python::with_gil(|py| {
                                    Ok::<PyObject, PyErr>(PyNone::get(py).as_unbound().clone_ref(py).into_any())    
                                })
                            })())?;
                            Ok::<PyObject, PyErr>(coroutine.unbind())
                        })
                    }
                })?;
                teo_wrap_builtin.call1((&model_object_class, set_name, set))?;
                // add
                let add_to_name = "add_to_".to_owned() + &base_relation_name;
                let add_to_name_cstr = static_cstr(&add_to_name)?;
                let add_to = PyCFunction::new_closure(py, Some(add_to_name_cstr), None, {
                    let relation_name = relation_name.clone();
                    move |args, _kwargs| {
                        let relation_name = relation_name.clone();
                        Python::with_gil(|py| {
                            let slf = args.get_item(0)?;
                            let model_object_wrapper: ModelObjectWrapper = slf.getattr("__teo_object__")?.extract()?;
                            let mut objects = vec![];
                            if args.len()? > 1 {
                                let binding = args.get_item(1)?;
                                let py_list: &Bound<PyList> = binding.downcast()?;
                                for item in py_list {
                                    objects.push(teo_model_object_from_py_model_object(py, item.unbind())?);
                                }
                            }
                            let coroutine = pyo3_async_runtimes::tokio::future_into_py::<_, PyObject>(py, (|| async move {
                                model_object_wrapper.object.force_add_relation_objects(&relation_name, objects).await;
                                Python::with_gil(|py| {
                                    Ok::<PyObject, PyErr>(PyNone::get(py).as_unbound().clone_ref(py).into_any())    
                                })
                            })())?;
                            Ok::<PyObject, PyErr>(coroutine.unbind())
                        })
                    }
                })?;
                teo_wrap_builtin.call1((&model_object_class, add_to_name, add_to))?;
                // remove
                let remove_from_name = "remove_from_".to_owned() + &base_relation_name;
                let remove_from_name_cstr = static_cstr(&remove_from_name)?;
                let remove_from = PyCFunction::new_closure(py, Some(remove_from_name_cstr), None, {
                    let relation_name = relation_name.clone();
                    move |args, _kwargs| {
                        let relation_name = relation_name.clone();
                        Python::with_gil(|py| {
                            let slf = args.get_item(0)?;
                            let model_object_wrapper: ModelObjectWrapper = slf.getattr("__teo_object__")?.extract()?;
                            let mut objects = vec![];
                            if args.len()? > 1 {
                                let binding = args.get_item(1)?;
                                let py_list: &Bound<PyList> = binding.downcast()?;
                                for item in py_list {
                                    objects.push(teo_model_object_from_py_model_object(py, item.unbind())?);
                                }
                            }
                            let coroutine = pyo3_async_runtimes::tokio::future_into_py::<_, PyObject>(py, (move || {
                                async move {
                                    model_object_wrapper.object.force_remove_relation_objects(&relation_name, objects).await;
                                    Python::with_gil(|py| {
                                        Ok::<PyObject, PyErr>(PyNone::get(py).as_unbound().clone_ref(py).into_any())    
                                    })    
                                }
                            })())?;
                            Ok::<PyObject, PyErr>(coroutine.unbind())
                        })
                    }
                })?;
                teo_wrap_builtin.call1((&model_object_class, remove_from_name, remove_from))?;
            } else {
                // to single
                // get
                let get = PyCFunction::new_closure(py, Some(base_relation_name_cstr), None, {
                    let relation_name = relation_name.clone();
                    let app_data = app_data.clone();
                    move |args, _kwargs| {
                        let relation_name = relation_name.clone();
                        let app_data = app_data.clone();
                        Python::with_gil(|py| {
                            let map = DynamicClasses::retrieve(&app_data)?;
                            let slf = args.get_item(0)?;
                            let model_object_wrapper: ModelObjectWrapper = slf.getattr("__teo_object__")?.extract()?;
                            let coroutine = pyo3_async_runtimes::tokio::future_into_py::<_, PyObject>(py, (move || {
                                async move {
                                    let value: Option<model::Object> = model_object_wrapper.object.force_get_relation_object(&relation_name).await?;
                                    Python::with_gil(|py| {
                                        match value {
                                            Some(value) => Ok::<PyObject, PyErr>(teo_model_object_to_py_any(py, &value, &map)?),
                                            None => Ok::<PyObject, PyErr>(PyNone::get(py).as_unbound().clone_ref(py).into_any())  
                                        }
                                    })    
                                }
                            })())?;
                            Ok::<PyObject, PyErr>(coroutine.unbind())
                        })
                    }
                })?;
                teo_wrap_builtin.call1((&model_object_class, &base_relation_name, get))?;
                // set
                let set_name = "set_".to_owned() + &base_relation_name;
                let set_name_c = static_cstr(&set_name)?;
                let set = PyCFunction::new_closure(py, Some(set_name_c), None, {
                    let relation_name = relation_name.clone();
                    move |args, _kwargs| {
                        let relation_name = relation_name.clone();
                        Python::with_gil(|py| {
                            let slf = args.get_item(0)?;
                            let model_object_wrapper: ModelObjectWrapper = slf.getattr("__teo_object__")?.extract()?;
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
                            let coroutine = pyo3_async_runtimes::tokio::future_into_py::<_, PyObject>(py, (move || async move {
                                model_object_wrapper.object.force_set_relation_object(&relation_name, next_argument).await;
                                Python::with_gil(|py| {
                                    Ok::<PyObject, PyErr>(PyNone::get(py).as_unbound().clone_ref(py).into_any())    
                                })
                            })())?;
                            Ok::<PyObject, PyErr>(coroutine.unbind())
                        })
                    }
                })?;
                teo_wrap_builtin.call1((&model_object_class, set_name, set))?;
            }
        }
        // properties
        for model_property in model.properties().values() {
            let field_name = model_property.name().to_string();
            if model_property.setter().is_some() {
                let name = "set_".to_owned() + &field_name.to_snake_case();
                let name_cstr = static_cstr(&name)?;
                let setter = PyCFunction::new_closure(py, Some(name_cstr), None, {
                    let field_name = field_name.clone();
                    move |args, _kwargs| {
                        let field_name = field_name.clone();
                        Python::with_gil(move |py| {
                            let slf = args.get_item(0)?;
                            let value = args.get_item(1)?;
                            let value = py_any_to_teo_value(py, &value)?;
                            let model_object_wrapper: ModelObjectWrapper = slf.getattr("__teo_object__")?.extract()?;
                            let coroutine = pyo3_async_runtimes::tokio::future_into_py::<_, PyObject>(py, (move || {
                                let field_name = field_name.clone();
                                async move {
                                    let _: () = model_object_wrapper.object.set_property(&field_name, value).await?;
                                    Python::with_gil(|py| {
                                        Ok(PyNone::get(py).as_unbound().clone_ref(py).into_any())
                                    })    
                                }
                            })())?;
                            Ok::<PyObject, PyErr>(coroutine.unbind())
                        })
                    }
                })?;
                teo_wrap_builtin.call1((&model_object_class, name, setter))?;
            }
            if model_property.getter().is_some() {
                let name = field_name.to_snake_case();
                let name_cstr = static_cstr(&name)?;
                let getter = PyCFunction::new_closure(py, Some(name_cstr), None, {
                    let field_name = field_name.clone();
                    let app_data = app_data.clone();
                    move |args, _kwargs| {
                        let field_name = field_name.clone();
                        let app_data = app_data.clone();
                        Python::with_gil(move |py| {
                            let map = DynamicClasses::retrieve(&app_data)?;
                            let slf = args.get_item(0)?;
                            let model_object_wrapper: ModelObjectWrapper = slf.getattr("__teo_object__")?.extract()?;
                            let coroutine = pyo3_async_runtimes::tokio::future_into_py::<_, PyObject>(py, (move || {
                                let field_name = field_name.clone();
                                async move {
                                    let result = model_object_wrapper.object.get_property_value(&field_name).await?;
                                    Python::with_gil(|py| {
                                        let any = teo_value_to_py_any(py, &result, &map)?;
                                        Ok(any)
                                    })    
                                }
                            })())?;
                            Ok::<PyObject, PyErr>(coroutine.unbind())
                        })
                    }
                })?;
                teo_wrap_builtin.call1((&model_object_class, name, getter))?;
            }
        }
    }
    for namespace in namespace.namespaces().values() {
        let namespace_name = namespace.path().join(".");
        let namespace_name_cstr = static_cstr(&namespace_name)?;
        let app_data = app_data.clone();
        let namespace_property = PyCFunction::new_closure(py, Some(namespace_name_cstr), None, move |args, _kwargs| {
            let app_data = app_data.clone();
            let next_ctx_object = Python::with_gil(|py| {
                let map = DynamicClasses::retrieve(&app_data)?;
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
    let transaction = PyCFunction::new_closure(py, Some(c"transaction"), Some(c"Run transaction."), move |args, _kwargs| {
        let app_data = app_data.clone();
        Python::with_gil(move |py| {
            let app_data = app_data.clone();
            let slf = args.get_item(0)?;
            let transaction_ctx_wrapper: TransactionCtxWrapper = slf.getattr("__teo_transaction_ctx__")?.extract()?;
            let argument = args.get_item(1)?.unbind();
            let coroutine = pyo3_async_runtimes::tokio::future_into_py::<_, PyObject>(py, (move || {
                let app_data = app_data.clone();
                let argument = Python::with_gil(|py| {
                    argument.clone_ref(py)
                });
                async move {
                    let retval: PyObject = transaction_ctx_wrapper.ctx.run_transaction(move |ctx: transaction::Ctx| {
                        let app_data = app_data.clone();
                        let argument = Python::with_gil(|py| {
                            argument.clone_ref(py)
                        });
                        async move {
                            let user_retval = Python::with_gil(move |py| {
                                let map = DynamicClasses::retrieve(&app_data)?;
                                let ctx_python = map.teo_transaction_ctx_to_py_ctx_object(py, ctx, "")?;
                                let coroutine_or_value = argument.call1(py, (ctx_python,))?;
                                let thread_locals = pyo3_async_runtimes::tokio::get_current_locals(py)?;
                                Ok::<(PyObject, TaskLocals), teo::prelude::Error>((coroutine_or_value, thread_locals))
                            })?;
                            Ok(await_coroutine_if_needed_value_with_locals(&user_retval.0, &user_retval.1).await?)    
                        }
                    }).await?;
                    Ok(retval)    
                }
            })())?;
            Ok::<PyObject, PyErr>(coroutine.unbind())
        })
    })?;
    teo_wrap_builtin.call1((&ctx_class, "transaction", transaction))?;
    // finish
    ctx_class.setattr(py, "__teo_initialized__", true)?;
    Ok(())
}


fn find_unique_object_function<'py>(py: Python<'py>, app_data: &AppData) -> PyResult<Bound<'py, PyCFunction>> {
    let app_data = app_data.clone();
    Ok(PyCFunction::new_closure(py, Some(c"find_unique_object"), Some(c"Find a unique object."), move |args, _kwargs| {
        let app_data = app_data.clone();
        Python::with_gil(move |py| {
            let app_data = app_data.clone();
            let slf = args.get_item(0)?;
            let model_ctx_wrapper: ModelCtxWrapper = slf.getattr("__teo_model_ctx__")?.extract()?;
            let find_many_arg = if args.len()? > 1 {
                let py_dict = args.get_item(1)?;
                check_py_dict(&py_dict)?;
                py_any_to_teo_value(py, &py_dict)?
            } else {
                Value::Dictionary(IndexMap::new())
            };
            let coroutine = pyo3_async_runtimes::tokio::future_into_py(py, (move || {
                let app_data = app_data.clone();
                async move {
                    let result: Option<model::Object> = model_ctx_wrapper.ctx.find_unique(&find_many_arg).await?;
                    Python::with_gil(move |py| {
                        let app_data = app_data.clone();
                        match result {
                            Some(object) => {
                                let map = DynamicClasses::retrieve(&app_data)?;
                                map.teo_model_object_to_py_model_object_object(py, object)
                            }
                            None => {
                                Ok(PyNone::get(py).as_unbound().clone_ref(py).into_any())
                            }
                        }
                    })    
                }
            })())?;
            Ok::<PyObject, PyErr>(coroutine.unbind())
        })
    })?)
}

fn find_first_object_function<'py>(py: Python<'py>, app_data: &AppData) -> PyResult<Bound<'py, PyCFunction>> {
    let app_data = app_data.clone();
    Ok(PyCFunction::new_closure(py, Some(c"find_first_object"), Some(c"Find an object."), move |args, _kwargs| {
        let app_data = app_data.clone();
        Python::with_gil(move |py| {
            let app_data = app_data.clone();
            let slf = args.get_item(0)?;
            let model_ctx_wrapper: ModelCtxWrapper = slf.getattr("__teo_model_ctx__")?.extract()?;
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
                            let map = DynamicClasses::retrieve(&app_data)?;
                            map.teo_model_object_to_py_model_object_object(py, object)
                        }
                        None => {
                            Ok(PyNone::get(py).as_unbound().clone_ref(py).into_any())
                        }
                    }
                })
            })())?;
            Ok::<PyObject, PyErr>(coroutine.unbind())
        })
    })?)
}

fn find_many_objects_function<'py>(py: Python<'py>, app_data: &AppData) -> PyResult<Bound<'py, PyCFunction>> {
    let app_data = app_data.clone();
    Ok(PyCFunction::new_closure(py, Some(c"find_many_objects"), Some(c"Find many objects."), move |args, _kwargs| {
        let app_data = app_data.clone();
        Python::with_gil(move |py| {
            let app_data = app_data.clone();
            let slf = args.get_item(0)?;
            let model_ctx_wrapper: ModelCtxWrapper = slf.getattr("__teo_model_ctx__")?.extract()?;
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
                    let py_result = PyList::empty(py);
                    let map = DynamicClasses::retrieve(&app_data)?;
                    for object in result {
                        let instance = map.teo_model_object_to_py_model_object_object(py, object)?;
                        py_result.append(instance)?;
                    }
                    Ok(py_result.into_any().unbind())
                })
            })())?;
            Ok::<PyObject, PyErr>(coroutine.unbind())
        })
    })?)
}

fn create_object_function<'py>(py: Python<'py>, app_data: &AppData) -> PyResult<Bound<'py, PyCFunction>> {
    let app_data = app_data.clone();
    Ok(PyCFunction::new_closure(py, Some(c"create_object"), Some(c"Create a new object."), move |args, _kwargs| {
        let app_data = app_data.clone();
        Python::with_gil(move |py| {
            let app_data = app_data.clone();
            let slf = args.get_item(0)?;
            let model_ctx_wrapper: ModelCtxWrapper = slf.getattr("__teo_model_ctx__")?.extract()?;
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
                    let map = DynamicClasses::retrieve(&app_data)?;
                    let instance = map.teo_model_object_to_py_model_object_object(py, result)?;
                    Ok(instance)
                })
            })())?;
            Ok::<PyObject, PyErr>(coroutine.unbind())
        })
    })?)
}

fn count_objects_function<'py>(py: Python<'py>) -> PyResult<Bound<PyCFunction>> {
    Ok(PyCFunction::new_closure(py, Some(c"count_objects"), Some(c"Count records."), move |args, _kwargs| {
        Python::with_gil(|py| {
            let slf = args.get_item(0)?;
            let model_ctx_wrapper: ModelCtxWrapper = slf.getattr("__teo_model_ctx__")?.extract()?;
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
                    Ok(result.into_py_any(py)?)
                })
            })())?;
            Ok::<PyObject, PyErr>(coroutine.unbind())
        })
    })?)
}

fn count_fields_function<'py>(py: Python<'py>, app_data: &AppData) -> PyResult<Bound<'py, PyCFunction>> {
    let app_data = app_data.clone();
    Ok(PyCFunction::new_closure(py, Some(c"count_fields"), Some(c"Count records."), move |args, _kwargs| {
        let app_data = app_data.clone();
        Python::with_gil(|py| {
            let slf = args.get_item(0)?;
            let model_ctx_wrapper: ModelCtxWrapper = slf.getattr("__teo_model_ctx__")?.extract()?;
            let count_arg = if args.len()? > 1 {
                let py_dict = args.get_item(1)?;
                check_py_dict(&py_dict)?;
                py_any_to_teo_value(py, &py_dict)?
            } else {
                Value::Dictionary(IndexMap::new())
            };
            let coroutine = pyo3_async_runtimes::tokio::future_into_py::<_, PyObject>(py, (|| async move {
                let map = DynamicClasses::retrieve(&app_data)?;
                let result: Value = model_ctx_wrapper.ctx.count_fields(&count_arg).await?;
                Python::with_gil(|py| {
                    teo_value_to_py_any(py, &result, &map)
                })
            })())?;
            Ok::<PyObject, PyErr>(coroutine.unbind())
        })
    })?)
}

fn aggregate_function<'py>(py: Python<'py>, app_data: &AppData) -> PyResult<Bound<'py, PyCFunction>> {
    let app_data = app_data.clone();
    Ok(PyCFunction::new_closure(py, Some(c"aggregate"), Some(c"Aggregate on records."), move |args, _kwargs| {
        let app_data = app_data.clone();
        Python::with_gil(|py| {
            let slf = args.get_item(0)?;
            let model_ctx_wrapper: ModelCtxWrapper = slf.getattr("__teo_model_ctx__")?.extract()?;
            let aggregate_arg = if args.len()? > 1 {
                let py_dict = args.get_item(1)?;
                check_py_dict(&py_dict)?;
                py_any_to_teo_value(py, &py_dict)?
            } else {
                Value::Dictionary(IndexMap::new())
            };
            let coroutine = pyo3_async_runtimes::tokio::future_into_py::<_, PyObject>(py, (|| async move {
                let map = DynamicClasses::retrieve(&app_data)?;
                let result: Value = model_ctx_wrapper.ctx.aggregate(&aggregate_arg).await?;
                Python::with_gil(|py| {
                    teo_value_to_py_any(py, &result, &map)
                })
            })())?;
            Ok::<PyObject, PyErr>(coroutine.unbind())
        })
    })?)
}

fn group_by_function<'py>(py: Python<'py>, app_data: &AppData) -> PyResult<Bound<'py, PyCFunction>> {
    let app_data = app_data.clone();
    Ok(PyCFunction::new_closure(py, Some(c"group_by"), Some(c"Group by on records."), move |args, _kwargs| {
        let app_data = app_data.clone();
        Python::with_gil(|py| {
            let slf = args.get_item(0)?;
            let model_ctx_wrapper: ModelCtxWrapper = slf.getattr("__teo_model_ctx__")?.extract()?;
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
                    let map = DynamicClasses::retrieve(&app_data)?;
                    Ok(teo_array_to_py_any(py, &result, &map)?)
                })
            })())?;
            Ok::<PyObject, PyErr>(coroutine.unbind())
        })
    })?)
}

fn sql_function<'py>(py: Python<'py>, app_data: &AppData) -> PyResult<Bound<'py, PyCFunction>> {
    let app_data = app_data.clone();
    Ok(PyCFunction::new_closure(py, Some(c"sql"), Some(c"Run custom SQL clause."), move |args, _kwargs| {
        let app_data = app_data.clone();
        Python::with_gil(|py| {
            let slf = args.get_item(0)?;
            let model_ctx_wrapper: ModelCtxWrapper = slf.getattr("__teo_model_ctx__")?.extract()?;
            let sql_string_any: Bound<PyAny> = args.get_item(1)?;
            let sql_string: String = sql_string_any.extract()?;
            let coroutine = pyo3_async_runtimes::tokio::future_into_py::<_, PyObject>(py, (|| async move {
                let result: Vec<Value> = model_ctx_wrapper.ctx.sql(&sql_string).await?;
                Python::with_gil(|py| {
                    let py_result = PyList::empty(py);
                    let map = DynamicClasses::retrieve(&app_data)?;
                    for value in result {
                        let instance = teo_value_to_py_any(py, &value, &map)?;
                        py_result.append(instance)?;
                    }
                    Ok(py_result.into_py_any(py)?)
                })
            })())?;
            Ok::<PyObject, PyErr>(coroutine.unbind())
        })
    })?)
}

fn is_new_function<'py>(py: Python<'py>) -> PyResult<Bound<PyCFunction>> {
    Ok(PyCFunction::new_closure(py, Some(c"is_new"), Some(c"Whether this model object is new."), move |args, _kwargs| {
        Python::with_gil(|py| {
            let slf = args.get_item(0)?;
            let model_object_wrapper: ModelObjectWrapper = slf.getattr("__teo_object__")?.extract()?;
            Ok::<PyObject, PyErr>(model_object_wrapper.object.is_new().into_py_any(py)?)
        })
    })?)
}

fn is_modified_function<'py>(py: Python<'py>) -> PyResult<Bound<PyCFunction>> {
    Ok(PyCFunction::new_closure(py, Some(c"is_modified"), Some(c"Whether this model object is modified."), move |args, _kwargs| {
        Python::with_gil(|py| {
            let slf = args.get_item(0)?;
            let model_object_wrapper: ModelObjectWrapper = slf.getattr("__teo_object__")?.extract()?;
            Ok::<PyObject, PyErr>(model_object_wrapper.object.is_modified().into_py_any(py)?)
        })
    })?)
}

fn set_function<'py>(py: Python<'py>) -> PyResult<Bound<PyCFunction>> {
    Ok(PyCFunction::new_closure(py, Some(c"set"), Some(c"Set values to this object."), move |args, _kwargs| {
        Python::with_gil(|py| {
            let slf = args.get_item(0)?;
            let model_object_wrapper: ModelObjectWrapper = slf.getattr("__teo_object__")?.extract()?;
            let set_arg = if args.len()? > 1 {
                let py_dict = args.get_item(1)?;
                check_py_dict(&py_dict)?;
                py_any_to_teo_value(py, &py_dict)?
            } else {
                Value::Dictionary(IndexMap::new())
            };
            let coroutine = pyo3_async_runtimes::tokio::future_into_py::<_, PyObject>(py, (|| async move {
                model_object_wrapper.object.set_teon(&set_arg).await?;
                Python::with_gil(|py| {
                    Ok(PyNone::get(py).as_unbound().clone_ref(py).into_any())
                })
            })())?;
            Ok::<PyObject, PyErr>(coroutine.unbind())
        })
    })?)
}

fn update_function<'py>(py: Python<'py>) -> PyResult<Bound<PyCFunction>> {
    Ok(PyCFunction::new_closure(py, Some(c"update"), Some(c"Update values on this object."), move |args, _kwargs| {
        Python::with_gil(|py| {
            let slf = args.get_item(0)?;
            let model_object_wrapper: ModelObjectWrapper = slf.getattr("__teo_object__")?.extract()?;
            let set_arg = if args.len()? > 1 {
                let py_dict = args.get_item(1)?;
                check_py_dict(&py_dict)?;
                py_any_to_teo_value(py, &py_dict)?
            } else {
                Value::Dictionary(IndexMap::new())
            };
            let coroutine = pyo3_async_runtimes::tokio::future_into_py::<_, PyObject>(py, (|| async move {
                model_object_wrapper.object.update_teon(&set_arg).await?;
                Python::with_gil(|py| {
                    Ok(PyNone::get(py).as_unbound().clone_ref(py).into_any())
                })
            })())?;
            Ok::<PyObject, PyErr>(coroutine.unbind())
        })
    })?)
}

fn save_function<'py>(py: Python<'py>) -> PyResult<Bound<PyCFunction>> {
    Ok(PyCFunction::new_closure(py, Some(c"save"), Some(c"Save this object."), move |args, _kwargs| {
        Python::with_gil(|py| {
            let slf = args.get_item(0)?;
            let model_object_wrapper: ModelObjectWrapper = slf.getattr("__teo_object__")?.extract()?;
            let coroutine = pyo3_async_runtimes::tokio::future_into_py::<_, PyObject>(py, (|| async move {
                model_object_wrapper.object.save().await?;
                Python::with_gil(|py| {
                    Ok(PyNone::get(py).as_unbound().clone_ref(py).into_any())
                })
            })())?;
            Ok::<PyObject, PyErr>(coroutine.unbind())
        })
    })?)
}

fn delete_function(py: Python) -> PyResult<Bound<PyCFunction>> {
    Ok(PyCFunction::new_closure(py, Some(c"delete"), Some(c"Delete this object."), move |args, _kwargs| {
        Python::with_gil(|py| {
            let slf = args.get_item(0)?;
            let model_object_wrapper: ModelObjectWrapper = slf.getattr("__teo_object__")?.extract()?;
            let coroutine = pyo3_async_runtimes::tokio::future_into_py::<_, PyObject>(py, (|| async move {
                model_object_wrapper.object.delete().await?;
                Python::with_gil(|py| {
                    Ok(PyNone::get(py).as_unbound().clone_ref(py).into_any())
                })
            })())?;
            Ok::<PyObject, PyErr>(coroutine.unbind())
        })
    })?)
}

fn to_teon_function<'py>(py: Python<'py>, app_data: &AppData) -> PyResult<Bound<'py, PyCFunction>> {
    let app_data = app_data.clone();
    Ok(PyCFunction::new_closure(py, Some(c"to_teon"), Some(c"Convert this object to a Teon object."), move |args, _kwargs| {
        let app_data = app_data.clone();
        Python::with_gil(|py| {
            let slf = args.get_item(0)?;
            let model_object_wrapper: ModelObjectWrapper = slf.getattr("__teo_object__")?.extract()?;
            let coroutine = pyo3_async_runtimes::tokio::future_into_py::<_, PyObject>(py, (|| async move {
                let result: Value = model_object_wrapper.object.to_teon().await?;
                let map = DynamicClasses::retrieve(&app_data)?;
                Python::with_gil(|py| {
                    teo_value_to_py_any(py, &result, &map)
                })
            })())?;
            Ok::<PyObject, PyErr>(coroutine.unbind())
        })
    })?)
}

fn repr_function<'py>(py: Python<'py>, app_data: &AppData) -> PyResult<Bound<'py, PyCFunction>> {
    let app_data = app_data.clone();
    Ok(PyCFunction::new_closure(py, Some(c"__repr__"), None, move |args, _kwargs| {
        let app_data = app_data.clone();
        Python::with_gil(|py| {
            let slf = args.get_item(0)?;
            let model_object_wrapper: ModelObjectWrapper = slf.getattr("__teo_object__")?.extract()?;
            let result = PyDict::new(py);
            let value_map = model_object_wrapper.object.inner.value_map.lock().unwrap();
            let map = DynamicClasses::retrieve(&app_data)?;
            for (k, v) in value_map.iter() {
                result.set_item(k, teo_value_to_py_any(py, v, &map)?)?;
            }
            let dict_repr = result.call_method("__repr__", (), None)?;
            let dict_repr_str: &str = dict_repr.extract()?;
            let prefix = format!("{}(", model_object_wrapper.object.model().path().join("."));
            let suffix = ")";
            Ok::<PyObject, PyErr>(format!("{}{}{}", prefix, dict_repr_str, suffix).into_py_any(py)?)
        })
    })?)
}
