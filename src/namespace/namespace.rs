use pyo3::{pyclass, pymethods, types::PyCFunction, IntoPy, Py, PyErr, PyObject, PyResult, Python};
use teo::prelude::{handler::Group as TeoHandlerGroup, model::Field as TeoField, model::Property as TeoProperty, model::Relation as TeoRelation, request, Enum as TeoEnum, Member as TeoEnumMember, Middleware, Model as TeoModel, Namespace as TeoNamespace, Next};

use crate::{utils::{check_callable::check_callable, await_coroutine_if_needed::await_coroutine_if_needed}, object::{arguments::teo_args_to_py_args, value::teo_value_to_py_any}, model::{model::Model, field::field::Field, relation::relation::Relation, property::property::Property}, result::{IntoPyResultWithGil, IntoTeoPathResult, IntoTeoResult}, r#enum::{r#enum::Enum, member::member::EnumMember}, request::{Request, RequestCtx}, dynamic::{py_ctx_object_from_teo_transaction_ctx, teo_transaction_ctx_from_py_ctx_object}, response::Response, handler::group::HandlerGroup};

#[pyclass]
pub struct Namespace {
    pub(crate) teo_namespace: &'static mut TeoNamespace,
}

#[pymethods]
impl Namespace {

    pub fn is_main(&self) -> bool {
        self.teo_namespace.is_main()
    }

    pub fn is_std(&self) -> bool {
        self.teo_namespace.is_std()
    }

    pub fn path(&self) -> Vec<&str> {
        self.teo_namespace.path()
    }

    pub fn namespace(&mut self, name: String) -> Option<Namespace> {
        let static_self: &'static mut Namespace = unsafe { &mut *(self as * const Namespace as * mut Namespace) };
        static_self.teo_namespace.namespace_mut(name.as_str()).map(|n| Namespace { teo_namespace: n })
    }

    pub fn namespace_or_create(&mut self, name: String) -> Namespace {
        let static_self: &'static mut Namespace = unsafe { &mut *(self as * const Namespace as * mut Namespace) };
        Namespace { teo_namespace: static_self.teo_namespace.namespace_mut_or_create(name.as_str()) }
    }

    pub fn namespace_at_path(&mut self, path: Vec<&str>) -> Option<Namespace> {
        let static_self: &'static mut Namespace = unsafe { &mut *(self as * const Namespace as * mut Namespace) };
        static_self.teo_namespace.namespace_mut_at_path(&path).map(|n| Namespace { teo_namespace: n })
    }

    pub fn namespace_or_create_at_path(&mut self, path: Vec<&str>) -> Namespace {
        let static_self: &'static mut Namespace = unsafe { &mut *(self as * const Namespace as * mut Namespace) };
        Namespace { teo_namespace: static_self.teo_namespace.namespace_mut_or_create_at_path(&path) }
    }

    pub fn define_model_decorator(&mut self, py: Python<'_>, name: &str, callback: PyObject) -> PyResult<()> {
        check_callable(callback.as_ref(py))?;
        let callback_owned = &*Box::leak(Box::new(Py::from(callback)));
        self.teo_namespace.define_model_decorator(name, |arguments, model| {
            Python::with_gil(|py| {
                let arguments = teo_args_to_py_args(py, &arguments)?;
                let static_model: &'static mut TeoModel = unsafe { &mut *(model as * mut TeoModel) };
                let model_wrapped = Model {
                    teo_model: static_model
                };
                callback_owned.call1(py, (arguments, model_wrapped))?;
                Ok::<(), PyErr>(())
            }).into_teo_result()?;
            Ok(())
        });
        Ok(())
    }

    pub fn define_model_field_decorator(&mut self, py: Python<'_>, name: &str, callback: PyObject) -> PyResult<()> {
        check_callable(callback.as_ref(py))?;
        let callback_owned = &*Box::leak(Box::new(Py::from(callback)));
        self.teo_namespace.define_model_field_decorator(name, |arguments, field| {
            Python::with_gil(|py| {
                let arguments = teo_args_to_py_args(py, &arguments)?;
                let static_field: &'static mut TeoField = unsafe { &mut *(field as * mut TeoField) };
                let model_wrapped = Field {
                    teo_field: static_field
                };
                callback_owned.call1(py, (arguments, model_wrapped))?;
                Ok::<(), PyErr>(())
            }).into_teo_result()?;
            Ok(())
        });
        Ok(())
    }

    pub fn define_model_relation_decorator(&mut self, py: Python<'_>, name: &str, callback: PyObject) -> PyResult<()> {
        check_callable(callback.as_ref(py))?;
        let callback_owned = &*Box::leak(Box::new(Py::from(callback)));
        self.teo_namespace.define_model_relation_decorator(name, |arguments, relation| {
            Python::with_gil(|py| {
                let arguments = teo_args_to_py_args(py, &arguments)?;
                let static_relation: &'static mut TeoRelation = unsafe { &mut *(relation as * mut TeoRelation) };
                let model_wrapped = Relation {
                    teo_relation: static_relation
                };
                callback_owned.call1(py, (arguments, model_wrapped))?;
                Ok::<(), PyErr>(())
            }).into_teo_result()?;
            Ok(())
        });
        Ok(())
    }

    pub fn define_model_property_decorator(&mut self, py: Python<'_>, name: &str, callback: PyObject) -> PyResult<()> {
        check_callable(callback.as_ref(py))?;
        let callback_owned = &*Box::leak(Box::new(Py::from(callback)));
        self.teo_namespace.define_model_property_decorator(name, |arguments, property| {
            Python::with_gil(|py| {
                let arguments = teo_args_to_py_args(py, &arguments)?;
                let static_property: &'static mut TeoProperty = unsafe { &mut *(property as * mut TeoProperty) };
                let model_wrapped = Property {
                    teo_property: static_property
                };
                callback_owned.call1(py, (arguments, model_wrapped))?;
                Ok::<(), PyErr>(())
            }).into_teo_result()?;
            Ok(())
        });
        Ok(())
    }

    pub fn define_enum_decorator(&mut self, py: Python<'_>, name: &str, callback: PyObject) -> PyResult<()> {
        check_callable(callback.as_ref(py))?;
        let callback_owned = &*Box::leak(Box::new(Py::from(callback)));
        self.teo_namespace.define_enum_decorator(name, |arguments, teo_enum: &mut TeoEnum| {
            Python::with_gil(|py| {
                let arguments = teo_args_to_py_args(py, &arguments)?;
                let static_teo_enum: &'static mut TeoEnum = unsafe { &mut *(teo_enum as * mut TeoEnum) };
                let model_wrapped = Enum {
                    teo_enum: static_teo_enum
                };
                callback_owned.call1(py, (arguments, model_wrapped))?;
                Ok::<(), PyErr>(())
            }).into_teo_result()?;
            Ok(())
        });
        Ok(())
    }

    pub fn define_enum_member_decorator(&mut self, py: Python<'_>, name: &str, callback: PyObject) -> PyResult<()> {
        check_callable(callback.as_ref(py))?;
        let callback_owned = &*Box::leak(Box::new(Py::from(callback)));
        self.teo_namespace.define_enum_member_decorator(name, |arguments, teo_enum: &mut TeoEnumMember| {
            Python::with_gil(|py| {
                let arguments = teo_args_to_py_args(py, &arguments)?;
                let static_teo_enum: &'static mut TeoEnumMember = unsafe { &mut *(teo_enum as * mut TeoEnumMember) };
                let model_wrapped = EnumMember {
                    teo_enum_member: static_teo_enum
                };
                callback_owned.call1(py, (arguments, model_wrapped))?;
                Ok::<(), PyErr>(())
            }).into_teo_result()?;
            Ok(())
        });
        Ok(())
    }

    pub fn define_handler(&mut self, py: Python<'_>, name: String, callback: PyObject) -> PyResult<()> {
        check_callable(callback.as_ref(py))?;
        let callback_owned = &*Box::leak(Box::new(Py::from(callback)));
        self.teo_namespace.define_handler(name.as_str(), move |ctx: request::Ctx| async move {
            let result: PyResult<_> = Python::with_gil(|py| {
                let request = Request {
                    teo_request: ctx.request().clone()
                };
                let body = teo_value_to_py_any(py, &ctx.body())?;
                let py_ctx = py_ctx_object_from_teo_transaction_ctx(py, ctx.transaction_ctx(), "")?;
                let result = callback_owned.call1(py, (request, body, py_ctx))?;
                let awaited_result = await_coroutine_if_needed(py, result.as_ref(py))?;
                let response: Response = awaited_result.extract(py)?;
                Ok(response.teo_response.clone())
            });
            result.into_teo_path_result()
        });
        Ok(())
    }

    pub fn define_handler_group(&mut self, py: Python<'_>, name: String, callback: PyObject) -> PyResult<()> {
        check_callable(callback.as_ref(py))?;
        self.teo_namespace.define_handler_group(name.as_str(), |teo_handler_group: &mut TeoHandlerGroup| {
            let static_model: &'static mut TeoHandlerGroup = unsafe { &mut *(teo_handler_group as * mut TeoHandlerGroup) };
            let handler_group = HandlerGroup { teo_handler_group: static_model };
            callback.call1(py, (handler_group,)).into_teo_result().unwrap();
        });
        Ok(())
    }

    pub fn define_model_handler_group(&mut self, py: Python<'_>, name: String, callback: PyObject) -> PyResult<()> {
        check_callable(callback.as_ref(py))?;
        self.teo_namespace.define_model_handler_group(name.as_str(), |teo_handler_group: &mut TeoHandlerGroup| {
            let static_model: &'static mut TeoHandlerGroup = unsafe { &mut *(teo_handler_group as * mut TeoHandlerGroup) };
            let handler_group = HandlerGroup { teo_handler_group: static_model };
            callback.call1(py, (handler_group,)).into_teo_result().unwrap();
        });
        Ok(())
    }

    pub fn define_middleware(&mut self, py: Python<'_>, name: String, callback: PyObject) -> PyResult<()> {
        let name = Box::leak(Box::new(name)).as_str();
        check_callable(callback.as_ref(py))?;
        let shared_callback = &*Box::leak(Box::new(callback));
        self.teo_namespace.define_middleware(name, move |arguments| async move {
            Python::with_gil(|py| {
                let py_args = teo_args_to_py_args(py, &arguments)?;
                let result_function = shared_callback.call1(py, (py_args,))?;
                let shared_result_function = &*Box::leak(Box::new(result_function));
                let wrapped_result = move |ctx: request::Ctx, next: &'static dyn Next| async move {
                    Python::with_gil(|py| {
                        let py_ctx = RequestCtx {
                            teo_inner: ctx
                        };
                        let py_next = PyCFunction::new_closure(py, Some(name), None, |args, _kwargs| {
                            Python::with_gil(|py| {
                                let ctx: RequestCtx = args.get_item(0)?.extract()?;
                                let teo_ctx = ctx.teo_inner.clone();
                                let coroutine = pyo3_asyncio::tokio::future_into_py::<_, PyObject>(py, (|| async {
                                    let result: teo::prelude::Response = next.call(teo_ctx).await.into_py_result_with_gil()?;
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

                        let coroutine = shared_result_function.call1(py, (py_ctx, py_next)).into_teo_result()?;
                        let result = await_coroutine_if_needed(py, coroutine.as_ref(py)).into_teo_result()?;
                        let response: Response = result.extract(py).into_teo_result()?;
                        Ok(response.teo_response)
                    })
                };
                let wrapped_box = Box::new(wrapped_result);
                let wrapped_raw = Box::leak(wrapped_box);
                let leak_static_result: &'static dyn Middleware = unsafe { &*(wrapped_raw as * const dyn Middleware) };
                return Ok(leak_static_result);    
            }).into_teo_result()
        });
        Ok(())
    }
}