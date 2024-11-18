use std::ffi::CString;

use pyo3::{pyclass, pymethods, types::{PyAnyMethods, PyCFunction}, Bound, IntoPy, Py, PyAny, PyErr, PyObject, PyResult, Python};
use pyo3_async_runtimes::TaskLocals;
use teo::prelude::{r#enum, handler, namespace, pipeline::{self, item::validator::Validity}, request, Middleware, MiddlewareImpl, Next, Value};
use teo_result::Error;

use crate::{dynamic::py_class_lookup_map::PYClassLookupMap, r#enum::{r#enum::Enum, member::member::EnumMember}, handler::group::HandlerGroup, model::{field::field::Field, model::Model, property::property::Property, relation::relation::Relation}, object::{arguments::teo_args_to_py_args, model::teo_model_object_to_py_any, value::{py_any_to_teo_value, teo_value_to_py_any}}, request::Request, response::Response, utils::{await_coroutine_if_needed::await_coroutine_if_needed_value_with_locals, check_callable::check_callable}};

#[pyclass]
pub struct Namespace {
    pub(crate) teo_namespace: namespace::Builder,
}

#[pymethods]
impl Namespace {

    pub fn is_main(&self) -> bool {
        self.teo_namespace.is_main()
    }

    pub fn is_std(&self) -> bool {
        self.teo_namespace.is_std()
    }

    pub fn path(&self) -> Vec<String> {
        self.teo_namespace.path().clone()
    }

    pub fn namespace(&self, name: String) -> Option<Namespace> {
        self.teo_namespace.namespace(name.as_str()).map(|n| Namespace { teo_namespace: n })
    }

    pub fn namespace_or_create(&self, name: String) -> Namespace {
        Namespace { teo_namespace: self.teo_namespace.namespace_or_create(name.as_str()) }
    }

    pub fn namespace_at_path(&self, path: Vec<String>) -> Option<Namespace> {
        self.teo_namespace.namespace_at_path(&path).map(|n| Namespace { teo_namespace: n })
    }

    pub fn namespace_or_create_at_path(&self, path: Vec<String>) -> Namespace {
        Namespace { teo_namespace: self.teo_namespace.namespace_or_create_at_path(&path) }
    }

    pub fn define_model_decorator(&self, py: Python<'_>, name: &str, callback: Bound<PyAny>) -> PyResult<()> {
        check_callable(&callback)?;
        let callback_owned = &*Box::leak(Box::new(Py::from(callback)));
        let map = PYClassLookupMap::from_app_data(self.teo_namespace.app_data());
        self.teo_namespace.define_model_decorator(name, |arguments, model| {
            Python::with_gil(|py| {
                let arguments = teo_args_to_py_args(py, &arguments, map)?;
                let model_wrapped = Model {
                    teo_model: model.clone()
                };
                callback_owned.call1(py, (arguments, model_wrapped))?;
                Ok::<(), PyErr>(())
            })?;
            Ok(())
        });
        Ok(())
    }

    pub fn define_model_field_decorator(&self, name: &str, callback: Bound<PyAny>) -> PyResult<()> {
        check_callable(&callback)?;
        let callback_owned = &*Box::leak(Box::new(Py::from(callback)));
        let map = PYClassLookupMap::from_app_data(self.teo_namespace.app_data());
        self.teo_namespace.define_model_field_decorator(name, |arguments, field| {
            Python::with_gil(|py| {
                let arguments = teo_args_to_py_args(py, &arguments, map)?;
                let field_wrapped = Field {
                    teo_field: field.clone()
                };
                callback_owned.call1(py, (arguments, field_wrapped))?;
                Ok::<(), PyErr>(())
            })?;
            Ok(())
        });
        Ok(())
    }

    pub fn define_model_relation_decorator(&self, name: &str, callback: Bound<PyAny>) -> PyResult<()> {
        check_callable(&callback)?;
        let callback_owned = &*Box::leak(Box::new(Py::from(callback)));
        let map = PYClassLookupMap::from_app_data(self.teo_namespace.app_data());
        self.teo_namespace.define_model_relation_decorator(name, |arguments, relation| {
            Python::with_gil(|py| {
                let arguments = teo_args_to_py_args(py, &arguments, map)?;
                let relation_wrapped = Relation {
                    teo_relation: relation.clone()
                };
                callback_owned.call1(py, (arguments, relation_wrapped))?;
                Ok::<(), PyErr>(())
            })?;
            Ok(())
        });
        Ok(())
    }

    pub fn define_model_property_decorator(&self, py: Python<'_>, name: &str, callback: Bound<PyAny>) -> PyResult<()> {
        check_callable(&callback)?;
        let callback_owned = &*Box::leak(Box::new(Py::from(callback)));
        let map = PYClassLookupMap::from_app_data(self.teo_namespace.app_data());
        self.teo_namespace.define_model_property_decorator(name, |arguments, property| {
            Python::with_gil(|py| {
                let arguments = teo_args_to_py_args(py, &arguments, map)?;
                let property_wrapped = Property {
                    teo_property: property.clone()
                };
                callback_owned.call1(py, (arguments, property_wrapped))?;
                Ok::<(), PyErr>(())
            })?;
            Ok(())
        });
        Ok(())
    }

    pub fn define_enum_decorator(&self, py: Python<'_>, name: &str, callback: Bound<PyAny>) -> PyResult<()> {
        check_callable(&callback)?;
        let callback_owned = &*Box::leak(Box::new(Py::from(callback)));
        let map = PYClassLookupMap::from_app_data(self.teo_namespace.app_data());
        self.teo_namespace.define_enum_decorator(name, |arguments, teo_enum: &r#enum::Builder| {
            Python::with_gil(|py| {
                let arguments = teo_args_to_py_args(py, &arguments, map)?;
                let enum_wrapped = Enum {
                    teo_enum: teo_enum.clone()
                };
                callback_owned.call1(py, (arguments, enum_wrapped))?;
                Ok::<(), PyErr>(())
            })?;
            Ok(())
        });
        Ok(())
    }

    pub fn define_enum_member_decorator(&self, py: Python<'_>, name: &str, callback: Bound<PyAny>) -> PyResult<()> {
        check_callable(&callback)?;
        let callback_owned = &*Box::leak(Box::new(Py::from(callback)));
        let map = PYClassLookupMap::from_app_data(self.teo_namespace.app_data());
        self.teo_namespace.define_enum_member_decorator(name, |arguments, member: &r#enum::member::Builder| {
            Python::with_gil(|py| {
                let arguments = teo_args_to_py_args(py, &arguments, map)?;
                let enum_member_wrapped = EnumMember {
                    teo_enum_member: member.clone()
                };
                callback_owned.call1(py, (arguments, enum_member_wrapped))?;
                Ok::<(), PyErr>(())
            })?;
            Ok(())
        });
        Ok(())
    }

    pub fn define_pipeline_item(&self, py: Python<'_>, name: &str, callback: Bound<PyAny>) -> PyResult<()> {
        check_callable(&callback)?;
        let callback_owned = &*Box::leak(Box::new(Py::from(callback)));
        let map = PYClassLookupMap::from_app_data(self.teo_namespace.app_data());
        self.teo_namespace.define_pipeline_item(name, move |args, ctx: pipeline::Ctx| async move {
            let result = Python::with_gil(|py| {
                let value = teo_value_to_py_any(py, ctx.value(), map)?;
                let args = teo_args_to_py_args(py, &args, map)?;
                let object = teo_model_object_to_py_any(py, ctx.object(), map)?;
                let ctx = map.teo_transaction_ctx_to_py_ctx_object(py, ctx.transaction_ctx(), "")?;
                let result = callback_owned.call1(py, (value, args, object, ctx))?;
                let locals = pyo3_async_runtimes::tokio::get_current_locals(py)?;
                Ok::<_, Error>((result, locals))
            })?;
            let awaited_result = await_coroutine_if_needed_value_with_locals(&result.0, &result.1).await?;
            Python::with_gil(|py| {
                let result = py_any_to_teo_value(py, &awaited_result.into_bound(py))?;
                Ok(result)
            })
        });
        Ok(())
    }

    pub fn define_transform_pipeline_item(&self, py: Python<'_>, name: &str, callback: Bound<PyAny>) -> PyResult<()> {
        self.define_pipeline_item(py, name, callback)
    }

    pub fn define_validator_pipeline_item(&self, py: Python<'_>, name: &str, callback: Bound<PyAny>) -> PyResult<()> {
        check_callable(&callback)?;
        let callback_owned = &*Box::leak(Box::new(Py::from(callback)));
        let map = PYClassLookupMap::from_app_data(self.teo_namespace.app_data());
        self.teo_namespace.define_validator_pipeline_item(name, move |_: Value, args, ctx: pipeline::Ctx| async move {
            let result = Python::with_gil(|py| {
                let value = teo_value_to_py_any(py, ctx.value(), map)?;
                let args = teo_args_to_py_args(py, &args, map)?;
                let object = teo_model_object_to_py_any(py, ctx.object(), map)?;
                let ctx = map.teo_transaction_ctx_to_py_ctx_object(py, ctx.transaction_ctx(), "")?;
                let result = callback_owned.call1(py, (value, args, object, ctx))?;
                let locals = pyo3_async_runtimes::tokio::get_current_locals(py)?;
                Ok::<_, Error>((result, locals))
            })?;
            let awaited_result = await_coroutine_if_needed_value_with_locals(&result.0, &result.1).await?;
            Python::with_gil(|py| {
                let result = py_any_to_teo_value(py, &awaited_result.into_bound(py))?;
                Ok::<Validity, Error>(match result {
                    Value::String(s) => {
                        Validity::Invalid(s.to_owned())
                    },
                    Value::Bool(b) => if b {
                        Validity::Valid
                    } else {
                        Validity::Invalid("value is invalid".to_owned())
                    },
                    _ => Validity::Valid
                })
            })
        });
        Ok(())
    }

    pub fn define_callback_pipeline_item(&self, py: Python<'_>, name: &str, callback: Bound<PyAny>) -> PyResult<()> {
        check_callable(&callback)?;
        let callback_owned = &*Box::leak(Box::new(Py::from(callback)));
        let map = PYClassLookupMap::from_app_data(self.teo_namespace.app_data());
        self.teo_namespace.define_callback_pipeline_item(name, move |args, ctx: pipeline::Ctx| async move {
            let result = Python::with_gil(|py| {
                let value = teo_value_to_py_any(py, ctx.value(), map)?;
                let args = teo_args_to_py_args(py, &args, map)?;
                let object = teo_model_object_to_py_any(py, ctx.object(), map)?;
                let ctx = map.teo_transaction_ctx_to_py_ctx_object(py, ctx.transaction_ctx(), "")?;
                let result = callback_owned.call1(py, (value, args, object, ctx))?;
                let locals = pyo3_async_runtimes::tokio::get_current_locals(py)?;
                Ok::<_, Error>((result, locals))
            })?;
            let _ = await_coroutine_if_needed_value_with_locals(&result.0, &result.1).await?;
            Ok(())
        });
        Ok(())
    }

    pub fn define_compare_pipeline_item(&self, py: Python<'_>, name: &str, callback: Bound<PyAny>) -> PyResult<()> {
        check_callable(&callback)?;
        let callback_owned = &*Box::leak(Box::new(Py::from(callback)));
        let map = PYClassLookupMap::from_app_data(self.teo_namespace.app_data());
        self.teo_namespace.define_compare_pipeline_item(name, move |old: Value, new: Value, args, ctx: pipeline::Ctx| async move {
            let result = Python::with_gil(|py| {
                let value_old = teo_value_to_py_any(py, &old, map)?;
                let value_new = teo_value_to_py_any(py, &new, map)?;
                let args = teo_args_to_py_args(py, &args, map)?;
                let object = teo_model_object_to_py_any(py, ctx.object(), map)?;
                let ctx = map.teo_transaction_ctx_to_py_ctx_object(py, ctx.transaction_ctx(), "")?;
                let result = callback_owned.call1(py, (value_old, value_new, args, object, ctx))?;
                let locals = pyo3_async_runtimes::tokio::get_current_locals(py)?;
                Ok::<_, Error>((result, locals))
            })?;
            let awaited_result = await_coroutine_if_needed_value_with_locals(&result.0, &result.1).await?;
            Python::with_gil(|py| {
                let result = py_any_to_teo_value(py, &awaited_result.into_bound(py))?;
                Ok::<Validity, teo::prelude::Error>(match result {
                    Value::String(s) => {
                        Validity::Invalid(s.to_owned())
                    },
                    Value::Bool(b) => if b {
                        Validity::Valid
                    } else {
                        Validity::Invalid("value is invalid".to_owned())
                    },
                    _ => Validity::Valid
                })
            })
        });
        Ok(())
    }

    pub fn define_handler(&self, name: String, callback: Bound<PyAny>) -> PyResult<()> {
        check_callable(&callback)?;
        let callback_object = Py::from(callback);
        let callback_object_never_ends = &*Box::leak(Box::new(callback_object));
        self.teo_namespace.define_handler(name.as_str(), move |request: request::Request| async move {
            let result = Python::with_gil(|py| {
                let request = Request {
                    teo_request: request
                };
                let result = callback_object_never_ends.call1(py, (request,))?;
                let thread_locals = pyo3_async_runtimes::tokio::get_current_locals(py)?;
                Ok::<(PyObject, TaskLocals), PyErr>((result, thread_locals))
            })?;
            let awaited_result = await_coroutine_if_needed_value_with_locals(&result.0, &result.1).await?;
            Python::with_gil(|py| {
                let response: Response = awaited_result.extract(py)?;
                Ok(response.teo_response.clone())
            })
        });
        Ok(())
    }

    pub fn define_handler_group(&self, name: String, callback: Bound<PyAny>) -> PyResult<()> {
        check_callable(&callback)?;
        self.teo_namespace.define_handler_group(name.as_str(), |teo_handler_group: &handler::group::Builder| {
            let handler_group = HandlerGroup { teo_handler_group: teo_handler_group.clone() };
            callback.call1((handler_group,)).unwrap();
        });
        Ok(())
    }

    pub fn define_model_handler_group(&self, py: Python<'_>, name: String, callback: Bound<PyAny>) -> PyResult<()> {
        check_callable(&callback)?;
        self.teo_namespace.define_model_handler_group(name.as_str(), |teo_handler_group: &handler::group::Builder| {
            let handler_group = HandlerGroup { teo_handler_group: teo_handler_group.clone() };
            callback.call1((handler_group,)).unwrap();
        });
        Ok(())
    }

    pub fn define_middleware(&self, py: Python<'_>, name: String, callback: PyObject) -> PyResult<()> {
        let name = Box::leak(Box::new(name)).as_str();
        let name_c = Box::leak(Box::new(CString::new(name)?)).as_c_str();
        check_callable(&callback.bind(py))?;
        let shared_callback = &*Box::leak(Box::new(callback));
        let map = PYClassLookupMap::from_app_data(self.teo_namespace.app_data());
        self.teo_namespace.define_request_middleware(name, move |arguments| async move {
            Python::with_gil(|py| {
                let py_args = teo_args_to_py_args(py, &arguments, map)?;
                let result_function = shared_callback.call1(py, (py_args,))?;
                let main = py.import_bound("__main__")?;
                let teo_wrap_async = main.getattr("teo_wrap_async")?.into_py(py);
                let wrapped_result_function = teo_wrap_async.call1(py, (result_function,))?;
                let shared_result_function = &*Box::leak(Box::new(wrapped_result_function));
                let wrapped_result = move |request: request::Request, next: &'static dyn Next| async move {
                    let coroutine = Python::with_gil(|py| {
                        let py_ctx = Request {
                            teo_request: request
                        };
                        let py_next = PyCFunction::new_closure_bound(py, Some(name_c), None, move |args, _kwargs| {
                            Python::with_gil(|py| {
                                let arg0 = args.get_item(0)?;
                                let request: Request = arg0.extract()?;
                                let main_thread_locals = pyo3_async_runtimes::tokio::get_current_locals(py)?;
                                let coroutine = pyo3_async_runtimes::tokio::future_into_py_with_locals::<_, PyObject>(py, main_thread_locals, (|| async {
                                    let result: teo::prelude::Response = next.call(request.teo_request).await?;
                                    Python::with_gil(|py| {
                                        let response = Response {
                                            teo_response: result
                                        };
                                        Ok::<PyObject, PyErr>(response.into_py(py))    
                                    })
                                })())?;
                                Ok::<PyObject, PyErr>(coroutine.into_py(py))
                            })
                        }).unwrap();
                        let coroutine = shared_result_function.call1(py, (py_ctx, py_next))?;
                        Ok::<PyObject, teo::prelude::Error>(coroutine.into_py(py))
                    })?;
                    let main_thread_locals = Python::with_gil(|py| {
                        let locals = pyo3_async_runtimes::tokio::get_current_locals(py)?;
                        Ok::<_, PyErr>(locals)
                    })?;
                    let result = await_coroutine_if_needed_value_with_locals(&coroutine, &main_thread_locals).await?;
                    Python::with_gil(|py| {
                        let response: Response = result.extract(py)?;
                        Ok(response.teo_response)    
                    })
                };                
                return Ok(MiddlewareImpl::new(wrapped_result));    
            })
        });
        Ok(())
    }
}