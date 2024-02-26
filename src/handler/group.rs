use pyo3::{pyclass, pymethods, Py, PyErr, PyObject, PyResult, Python};
use teo::prelude::{handler::Group as TeoHandlerGroup, request};

use crate::dynamic::py_ctx_object_from_teo_transaction_ctx;
use crate::object::value::teo_value_to_py_any;
use crate::request::request::Request;
use crate::request::RequestCtx;
use crate::response::Response;
use crate::utils::await_coroutine_if_needed::await_coroutine_if_needed_value_with_locals;
use crate::utils::check_callable::check_callable;
use crate::result::IntoTeoResult;

#[pyclass]
pub struct HandlerGroup {
    pub(crate) teo_handler_group: &'static mut TeoHandlerGroup,
}

#[pymethods]
impl HandlerGroup {

    pub fn define_handler(&mut self, py: Python<'_>, name: String, callback: PyObject) -> PyResult<()> {
        check_callable(callback.as_ref(py))?;
        let main_thread_locals = &*Box::leak(Box::new(pyo3_asyncio::tokio::get_current_locals(py)?));
        let callback_owned = &*Box::leak(Box::new(Py::from(callback)));
        self.teo_handler_group.define_handler(name.as_str(), move |ctx: request::Ctx| async move {
            let result = Python::with_gil(|py| {
                let request_ctx = RequestCtx {
                    teo_inner: ctx
                };
                let result = callback_owned.call1(py, (request_ctx,))?;
                Ok::<PyObject, PyErr>(result)
            }).into_teo_result()?;
            let awaited_result = await_coroutine_if_needed_value_with_locals(result, main_thread_locals).await.into_teo_result()?;
            Python::with_gil(|py| {
                let response: Response = awaited_result.extract(py).into_teo_result()?;
                Ok(response.teo_response.clone())
            })
        });
        Ok(())
    }
}