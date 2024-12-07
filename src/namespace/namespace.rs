use teo_result::Error;
use pyo3::{pyclass, pymethods, types::{PyAnyMethods, PyCFunction}, Bound, IntoPyObjectExt, Py, PyAny, PyErr, PyObject, PyResult, Python};
use pyo3_async_runtimes::TaskLocals;
use teo::prelude::{r#enum, handler, namespace, pipeline::{self, item::templates::validator::Validity}, request, Middleware, Next, NextImp, Value};
use crate::{dynamic::DynamicClasses, r#enum::{r#enum::Enum, member::member::EnumMember}, handler::group::HandlerGroup, model::{field::field::Field, model::Model, property::property::Property, relation::relation::Relation}, object::{arguments::teo_args_to_py_args, value::{py_any_to_teo_value, teo_value_to_py_any}}, pipeline::ctx::PipelineCtx, request::Request, response::Response, utils::{await_coroutine_if_needed::await_coroutine_if_needed_value_with_locals, check_callable::check_callable, cstr::static_cstr}};
use teo::prelude::request::Request as TeoRequest;

#[pyclass]
pub struct Namespace {
    pub(crate) builder: namespace::Builder,
}

#[pymethods]
impl Namespace {

    #[getter]
    pub fn is_main(&self) -> bool {
        self.builder.is_main()
    }

    #[getter]
    pub fn is_std(&self) -> bool {
        self.builder.is_std()
    }

    #[getter]
    pub fn path(&self) -> &Vec<String> {
        self.builder.path()
    }

    pub fn namespace(&self, name: &str) -> Namespace {
        self.child_namespace_or_create(name)
    }

    pub fn child_namespace(&self, name: &str) -> Option<Namespace> {
        self.builder.child_namespace(name).map(|n| Namespace { builder: n })
    }

    pub fn child_namespace_or_create(&self, name: &str) -> Namespace {
        Namespace { builder: self.builder.child_namespace_or_create(name) }
    }

    pub fn descendant_namespace_at_path(&self, path: Vec<String>) -> Option<Namespace> {
        self.builder.descendant_namespace_at_path(&path).map(|n| Namespace { builder: n })
    }

    pub fn descendant_namespace_or_create_at_path(&self, path: Vec<String>) -> Namespace {
        Namespace { builder: self.builder.descendant_namespace_or_create_at_path(&path) }
    }

    pub fn define_model_decorator(&self, name: &str, callback: PyObject, py: Python<'_>) -> PyResult<()> {
        check_callable(callback.bind(py))?;
        self.builder.define_model_decorator(name, move |arguments, model| {
            Python::with_gil(|py| {
                let arguments = teo_args_to_py_args(py, &arguments)?;
                let model_wrapped = Model {
                    builder: model.clone()
                };
                callback.call1(py, (arguments, model_wrapped))?;
                Ok::<(), PyErr>(())
            })?;
            Ok(())
        });
        Ok(())
    }

    pub fn define_model_field_decorator(&self, name: &str, callback: PyObject, py: Python<'_>) -> PyResult<()> {
        check_callable(callback.bind(py))?;
        self.builder.define_model_field_decorator(name, move |arguments, field| {
            Python::with_gil(|py| {
                let arguments = teo_args_to_py_args(py, &arguments)?;
                let field_wrapped = Field {
                    builder: field.clone()
                };
                callback.call1(py, (arguments, field_wrapped))?;
                Ok::<(), PyErr>(())
            })?;
            Ok(())
        });
        Ok(())
    }

    pub fn define_model_relation_decorator(&self, name: &str, callback: PyObject, py: Python<'_>) -> PyResult<()> {
        check_callable(callback.bind(py))?;
        self.builder.define_model_relation_decorator(name, move |arguments, relation| {
            Python::with_gil(|py| {
                let arguments = teo_args_to_py_args(py, &arguments)?;
                let relation_wrapped = Relation {
                    builder: relation.clone()
                };
                callback.call1(py, (arguments, relation_wrapped))?;
                Ok::<(), PyErr>(())
            })?;
            Ok(())
        });
        Ok(())
    }

    pub fn define_model_property_decorator(&self, name: &str, callback: PyObject, py: Python<'_>) -> PyResult<()> {
        check_callable(callback.bind(py))?;
        self.builder.define_model_property_decorator(name, move |arguments, property| {
            Python::with_gil(|py| {
                let arguments = teo_args_to_py_args(py, &arguments)?;
                let property_wrapped = Property {
                    builder: property.clone()
                };
                callback.call1(py, (arguments, property_wrapped))?;
                Ok::<(), PyErr>(())
            })?;
            Ok(())
        });
        Ok(())
    }

    pub fn define_enum_decorator(&self, name: &str, callback: PyObject, py: Python<'_>) -> PyResult<()> {
        check_callable(callback.bind(py))?;
        self.builder.define_enum_decorator(name, move |arguments, teo_enum: &r#enum::Builder| {
            Python::with_gil(|py| {
                let arguments = teo_args_to_py_args(py, &arguments)?;
                let enum_wrapped = Enum {
                    builder: teo_enum.clone()
                };
                callback.call1(py, (arguments, enum_wrapped))?;
                Ok::<(), PyErr>(())
            })?;
            Ok(())
        });
        Ok(())
    }

    pub fn define_enum_member_decorator(&self, name: &str, callback: PyObject, py: Python<'_>) -> PyResult<()> {
        check_callable(callback.bind(py))?;
        self.builder.define_enum_member_decorator(name, move |arguments, member: &r#enum::member::Builder| {
            Python::with_gil(|py| {
                let arguments = teo_args_to_py_args(py, &arguments)?;
                let enum_member_wrapped = EnumMember {
                    builder: member.clone()
                };
                callback.call1(py, (arguments, enum_member_wrapped))?;
                Ok::<(), PyErr>(())
            })?;
            Ok(())
        });
        Ok(())
    }

    pub fn _define_pipeline_item(&self, name: &str, callback: PyObject, py: Python<'_>) -> PyResult<()> {
        check_callable(&callback.bind(py))?;
        let main_thread_locals = pyo3_async_runtimes::tokio::get_current_locals(py)?;
        self.builder.define_pipeline_item(name, move |args| {
            Python::with_gil(|py| {
                let args = teo_args_to_py_args(py, &args)?;
                let callback = callback.clone_ref(py);
                let main_thread_locals = main_thread_locals.clone_ref(py);
                let python_pipeline_item = callback.call1(py, (args,))?;
                return Ok(move |ctx: pipeline::Ctx| {
                    let gil_result = Python::with_gil(|py| {
                        let main_thread_locals = main_thread_locals.clone_ref(py);
                        let current_thread_locals_result = pyo3_async_runtimes::tokio::get_current_locals(py);
                        let thread_locals = if let Ok(current_thread_locals) = current_thread_locals_result {
                            current_thread_locals
                        } else {
                            main_thread_locals
                        };
                        let python_pipeline_item = python_pipeline_item.clone_ref(py);
                        let ctx = PipelineCtx::from(ctx);
                        let python_pipeline_item_result = python_pipeline_item.call1(py, (ctx,))?;
                        Ok::<_, Error>((thread_locals, python_pipeline_item_result))
                    });
                    async move {
                        let (thread_locals, python_pipeline_item_result) = gil_result?;
                        let python_result = await_coroutine_if_needed_value_with_locals(&python_pipeline_item_result, &thread_locals).await?;
                        Python::with_gil(|py| {
                            let bounded_result = python_result.into_bound(py);
                            Ok(py_any_to_teo_value(py, &bounded_result)?)
                        })
                    }
                })
            })
        });
        Ok(())
    }

    pub fn _define_validator_pipeline_item(&self, name: &str, callback: Bound<PyAny>, py: Python<'_>) -> PyResult<()> {
        check_callable(&callback)?;
        let main_thread_locals = pyo3_async_runtimes::tokio::get_current_locals(py)?;
        let callback = Py::from(callback);
        self.builder.define_pipeline_item(name, move |args| {
            Python::with_gil(|py| {
                let args = teo_args_to_py_args(py, &args)?;
                let callback = callback.clone_ref(py);
                let main_thread_locals = main_thread_locals.clone_ref(py);
                let python_pipeline_item = callback.call1(py, (args,))?;
                return Ok(move |ctx: pipeline::Ctx| {
                    let gil_result = Python::with_gil(|py| {
                        let main_thread_locals = main_thread_locals.clone_ref(py);
                        let current_thread_locals_result = pyo3_async_runtimes::tokio::get_current_locals(py);
                        let thread_locals = if let Ok(current_thread_locals) = current_thread_locals_result {
                            current_thread_locals
                        } else {
                            main_thread_locals
                        };
                        let python_pipeline_item = python_pipeline_item.clone_ref(py);
                        let ctx = PipelineCtx::from(ctx.clone());
                        let python_pipeline_item_result = python_pipeline_item.call1(py, (ctx,))?;
                        Ok::<_, Error>((thread_locals, python_pipeline_item_result))
                    });
                    async move {
                        let (thread_locals, python_pipeline_item_result) = gil_result?;
                        let python_result = await_coroutine_if_needed_value_with_locals(&python_pipeline_item_result, &thread_locals).await?;
                        let validity = Python::with_gil(|py| {
                            let bounded_result = python_result.into_bound(py);
                            let teo_result = py_any_to_teo_value(py, &bounded_result)?;
                            Ok::<Validity, Error>(match teo_result {
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
                        })?;
                        match validity {
                            Validity::Valid => Ok(ctx.value().clone()),
                            Validity::Invalid(reason) => Err(Error::new_with_code(reason, 400)),
                        }
                    }
                })
            })
        });
        Ok(())
    }

    pub fn _define_callback_pipeline_item(&self, name: &str, callback: Bound<PyAny>, py: Python<'_>) -> PyResult<()> {
        check_callable(&callback)?;
        let main_thread_locals = pyo3_async_runtimes::tokio::get_current_locals(py)?;
        let callback = Py::from(callback);
        self.builder.define_pipeline_item(name, move |args| {
            Python::with_gil(|py| {
                let args = teo_args_to_py_args(py, &args)?;
                let callback = callback.clone_ref(py);
                let main_thread_locals = main_thread_locals.clone_ref(py);
                let python_pipeline_item = callback.call1(py, (args,))?;
                return Ok(move |ctx: pipeline::Ctx| {
                    let gil_result = Python::with_gil(|py| {
                        let main_thread_locals = main_thread_locals.clone_ref(py);
                        let current_thread_locals_result = pyo3_async_runtimes::tokio::get_current_locals(py);
                        let thread_locals = if let Ok(current_thread_locals) = current_thread_locals_result {
                            current_thread_locals
                        } else {
                            main_thread_locals
                        };
                        let python_pipeline_item = python_pipeline_item.clone_ref(py);
                        let ctx = PipelineCtx::from(ctx.clone());
                        let python_pipeline_item_result = python_pipeline_item.call1(py, (ctx,))?;
                        Ok::<_, Error>((thread_locals, python_pipeline_item_result))
                    });
                    async move {
                        let (thread_locals, python_pipeline_item_result) = gil_result?;
                        let _ = await_coroutine_if_needed_value_with_locals(&python_pipeline_item_result, &thread_locals).await?;
                        Ok::<Value, Error>(ctx.value().clone())
                    }
                })
            })
        });
        Ok(())
    }

    pub fn _define_compare_pipeline_item(&self, name: &str, callback: Bound<PyAny>, py: Python<'_>) -> PyResult<()> {
        check_callable(&callback)?;
        let main_thread_locals = pyo3_async_runtimes::tokio::get_current_locals(py)?;
        let callback = Py::from(callback);
        self.builder.define_pipeline_item(name, move |args| {
            Python::with_gil(|py| {
                let args = teo_args_to_py_args(py, &args)?;
                let callback = callback.clone_ref(py);
                let main_thread_locals = main_thread_locals.clone_ref(py);
                let python_pipeline_item = callback.call1(py, (args,))?;
                return Ok(move |ctx: pipeline::Ctx| {
                    let app_data = ctx.object().namespace().app_data().clone();
                    let gil_result = Python::with_gil(|py| {
                        let main_thread_locals = main_thread_locals.clone_ref(py);
                        let current_thread_locals_result = pyo3_async_runtimes::tokio::get_current_locals(py);
                        let thread_locals = if let Ok(current_thread_locals) = current_thread_locals_result {
                            current_thread_locals
                        } else {
                            main_thread_locals
                        };
                        let python_pipeline_item = python_pipeline_item.clone_ref(py);
                        Ok::<_, Error>((thread_locals, python_pipeline_item))
                    });
                    async move {
                        let dynamic_classes = DynamicClasses::retrieve(&app_data)?;
                        let (thread_locals, python_pipeline_item) = gil_result?;
                        if ctx.object().is_new() {
                            return Ok(ctx.value().clone());
                        }
                        let key = ctx.path()[ctx.path().len() - 1].as_key().unwrap();
                        let previous_value = ctx.object().get_previous_value(key)?;
                        let current_value = ctx.value();
                        if &previous_value == current_value {
                            return Ok(ctx.value().clone());
                        }
                        let python_pipeline_item_result = Python::with_gil(|py| {
                            let ctx = PipelineCtx::from(ctx.clone());
                            let old_value_py = teo_value_to_py_any(py, &previous_value, &dynamic_classes)?;
                            let new_value_py = teo_value_to_py_any(py, current_value, &dynamic_classes)?;
                            let python_pipeline_item_result = python_pipeline_item.call1(py, (old_value_py, new_value_py, ctx))?;
                            Ok::<PyObject, Error>(python_pipeline_item_result)
                        })?;
                        let python_result = await_coroutine_if_needed_value_with_locals(&python_pipeline_item_result, &thread_locals).await?;
                        let validity = Python::with_gil(|py| {
                            let bounded_result = python_result.into_bound(py);
                            let teo_result = py_any_to_teo_value(py, &bounded_result)?;
                            Ok::<Validity, Error>(match teo_result {
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
                        })?;
                        match validity {
                            Validity::Valid => Ok(ctx.value().clone()),
                            Validity::Invalid(reason) => Err(Error::new_with_code(reason, 400)),
                        }
                    }
                })
            })
        });
        Ok(())
    }

    pub fn _define_handler(&self, name: String, callback: PyObject, py: Python<'_>) -> PyResult<()> {
        check_callable(callback.bind(py))?;
        let main_thread_locals = pyo3_async_runtimes::tokio::get_current_locals(py)?;
        let callback_object = Py::from(callback);
        self.builder.define_handler(name.as_str(), move |request: TeoRequest| {
            let (main_thread_locals, callback_object) = Python::with_gil(|py| {
                (main_thread_locals.clone_ref(py), callback_object.clone_ref(py))
            });
            async move {
                let (coroutine, thread_locals) = Python::with_gil(|py| {
                    let request = Request::from(request);
                    let result = callback_object.call1(py, (request,))?;
                    let current_thread_locals_result = pyo3_async_runtimes::tokio::get_current_locals(py);
                    if let Ok(current_thread_locals) = current_thread_locals_result {
                        Ok::<(PyObject, TaskLocals), PyErr>((result, current_thread_locals))
                    } else {
                        Ok::<(PyObject, TaskLocals), PyErr>((result, main_thread_locals))
                    }
                })?;
                let awaited_result = await_coroutine_if_needed_value_with_locals(&coroutine, &thread_locals).await?;
                Python::with_gil(|py| {
                    let response: Response = awaited_result.extract(py)?;
                    Ok(response.original.clone())
                })
            }
        });
        Ok(())
    }

    pub fn define_handler_group(&self, name: String, callback: Bound<PyAny>) -> PyResult<()> {
        check_callable(&callback)?;
        self.builder.define_handler_group(name.as_str(), |teo_handler_group: &handler::group::Builder| {
            let handler_group = HandlerGroup { teo_handler_group: teo_handler_group.clone() };
            callback.call1((handler_group,))?;
            Ok(())
        })?;
        Ok(())
    }

    pub fn handler_group(&self, name: &str) -> Option<HandlerGroup> {
        self.builder.handler_group(name).map(|hg| HandlerGroup { teo_handler_group: hg.clone() })
    }

    pub fn group(&self, name: &str) -> HandlerGroup {
        HandlerGroup { teo_handler_group: self.builder.handler_group_or_create(name) }
    }

    pub fn define_model_handler_group(&self, name: String, callback: Bound<PyAny>) -> PyResult<()> {
        check_callable(&callback)?;
        self.builder.define_model_handler_group(name.as_str(), |teo_handler_group: &handler::group::Builder| {
            let handler_group = HandlerGroup { teo_handler_group: teo_handler_group.clone() };
            callback.call1((handler_group,))?;
            Ok(())
        })?;
        Ok(())
    }

    pub fn model_handler_group(&self, name: &str) -> Option<HandlerGroup> {
        self.builder.model_handler_group(name).map(|hg| HandlerGroup { teo_handler_group: hg.clone() })
    }

    pub fn model(&self, name: &str) -> HandlerGroup {
        HandlerGroup { teo_handler_group: self.builder.model_handler_group_or_create(name) }
    }

    pub fn _define_request_middleware(&self, py: Python<'_>, name: &str, creator: PyObject) -> PyResult<()> {
        let name_cstr = static_cstr(name)?;
        check_callable(&creator.bind(py))?;
        let main_thread_locals = pyo3_async_runtimes::tokio::get_current_locals(py)?;
        self.builder.define_request_middleware(name, move |args| {
            Python::with_gil(|py| {
                let main_thread_locals = main_thread_locals.clone_ref(py);
                let py_args = teo_args_to_py_args(py, &args)?;
                let creator = creator.clone_ref(py);
                let python_middleware = creator.call1(py, (py_args,))?;
                Ok(move |request: request::Request, next: Next| {
                    let (
                         python_middleware, 
                         main_thread_locals, 
                         python_request, 
                    ) = Python::with_gil(|py| {
                        (
                            python_middleware.clone_ref(py), 
                            main_thread_locals.clone_ref(py),
                            Request::from(request),
                        )
                    });
                    async move {
                        let coroutine = Python::with_gil(|py| {
                            let python_next = PyCFunction::new_closure(py, Some(name_cstr), None, move |args, _kwargs| {
                                let next = next.clone();
                                Python::with_gil(|py| {
                                    let arg0 = args.get_item(0)?;
                                    let request: Request = arg0.extract()?;
                                    let coroutine = pyo3_async_runtimes::tokio::future_into_py::<_, PyObject>(py, (move || {
                                        let next = next.clone();
                                        async move {
                                            let original_response: teo::prelude::Response = next.call(request.original).await?;
                                            Python::with_gil(|py| {
                                                let response = Response::from(original_response);
                                                Ok::<PyObject, PyErr>(response.into_py_any(py)?)    
                                            })
                                        }
                                    })())?;
                                    Ok::<PyObject, PyErr>(coroutine.unbind())
                                })
                            })?;
                            let coroutine = python_middleware.call1(py, (python_request, python_next))?;
                            Ok::<PyObject, teo::prelude::Error>(coroutine)
                        })?;
                        let thread_locals = Python::with_gil(|py| {
                            if let Ok(local_thread_locals) = pyo3_async_runtimes::tokio::get_current_locals(py) {
                                local_thread_locals
                            } else {
                                main_thread_locals
                            }
                        });
                        let result = await_coroutine_if_needed_value_with_locals(&coroutine, &thread_locals).await?;
                        Python::with_gil(|py| {
                            let response: Response = result.extract(py)?;
                            Ok(response.original)    
                        })
                    }
                })
            })
        });
        Ok(())
    }

    pub fn _define_handler_middleware(&self, py: Python<'_>, name: &str, creator: PyObject) -> PyResult<()> {
        let name_cstr = static_cstr(name)?;
        check_callable(&creator.bind(py))?;
        let main_thread_locals = pyo3_async_runtimes::tokio::get_current_locals(py)?;
        self.builder.define_handler_middleware(name, move |args| {
            Python::with_gil(|py| {
                let main_thread_locals = main_thread_locals.clone_ref(py);
                let py_args = teo_args_to_py_args(py, &args)?;
                let creator = creator.clone_ref(py);
                let python_middleware = creator.call1(py, (py_args,))?;
                Ok(move |request: request::Request, next: Next| {
                    let (
                         python_middleware, 
                         main_thread_locals, 
                         python_request, 
                    ) = Python::with_gil(|py| {
                        (
                            python_middleware.clone_ref(py), 
                            main_thread_locals.clone_ref(py),
                            Request::from(request),
                        )
                    });
                    async move {
                        let coroutine = Python::with_gil(|py| {
                            let python_next = PyCFunction::new_closure(py, Some(name_cstr), None, move |args, _kwargs| {
                                let next = next.clone();
                                Python::with_gil(|py| {
                                    let arg0 = args.get_item(0)?;
                                    let request: Request = arg0.extract()?;
                                    let coroutine = pyo3_async_runtimes::tokio::future_into_py::<_, PyObject>(py, (move || {
                                        let next = next.clone();
                                        async move {
                                            let original_response: teo::prelude::Response = next.call(request.original).await?;
                                            Python::with_gil(|py| {
                                                let response = Response::from(original_response);
                                                Ok::<PyObject, PyErr>(response.into_py_any(py)?)    
                                            })
                                        }
                                    })())?;
                                    Ok::<PyObject, PyErr>(coroutine.unbind())
                                })
                            })?;
                            let coroutine = python_middleware.call1(py, (python_request, python_next))?;
                            Ok::<PyObject, teo::prelude::Error>(coroutine)
                        })?;
                        let thread_locals = Python::with_gil(|py| {
                            if let Ok(local_thread_locals) = pyo3_async_runtimes::tokio::get_current_locals(py) {
                                local_thread_locals
                            } else {
                                main_thread_locals
                            }
                        });
                        let result = await_coroutine_if_needed_value_with_locals(&coroutine, &thread_locals).await?;
                        Python::with_gil(|py| {
                            let response: Response = result.extract(py)?;
                            Ok(response.original)    
                        })
                    }
                })
            })
        });
        Ok(())
    }
}