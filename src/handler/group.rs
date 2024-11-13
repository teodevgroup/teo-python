use pyo3::{pyclass, pymethods, Bound, Py, PyAny, PyErr, PyObject, PyResult, Python};
use teo::prelude::handler;
use teo::prelude::request;
use crate::request::RequestCtx;
use crate::response::Response;
use crate::utils::await_coroutine_if_needed::await_coroutine_if_needed_value_with_locals;
use crate::utils::check_callable::check_callable;

#[pyclass]
pub struct HandlerGroup {
    pub(crate) teo_handler_group: handler::group::Builder,
}

#[pymethods]
impl HandlerGroup {

    pub fn define_handler(&mut self, py: Python<'_>, name: String, callback: Bound<PyAny>) -> PyResult<()> {
        check_callable(&callback)?;
        let main_thread_locals = &*Box::leak(Box::new(pyo3_async_runtimes::tokio::get_current_locals(py)?));
        let callback_owned = &*Box::leak(Box::new(Py::from(callback)));
        self.teo_handler_group.define_handler(name.as_str(), move |ctx: request::Ctx| async move {
            let result = Python::with_gil(|py| {
                let request_ctx = RequestCtx {
                    teo_inner: ctx
                };
                let result = callback_owned.call1(py, (request_ctx,))?;
                Ok::<PyObject, PyErr>(result)
            })?;
            let awaited_result = await_coroutine_if_needed_value_with_locals(result, main_thread_locals).await?;
            Python::with_gil(|py| {
                let response: Response = awaited_result.extract(py)?;
                Ok(response.teo_response.clone())
            })
        });
        Ok(())
    }
}