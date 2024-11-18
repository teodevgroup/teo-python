use pyo3::types::PyCapsule;
use pyo3::{pyclass, pymethods, Bound, Py, PyAny, PyErr, PyObject, PyResult, Python};
use pyo3_async_runtimes::TaskLocals;
use teo::prelude::handler;
use teo::prelude::request::Request as TeoRequest;
use crate::request::Request;
use crate::response::Response;
use crate::utils::await_coroutine_if_needed::await_coroutine_if_needed_value_with_locals;
use crate::utils::check_callable::check_callable;

#[pyclass]
pub struct HandlerGroup {
    pub(crate) teo_handler_group: handler::group::Builder,
}

#[pymethods]
impl HandlerGroup {

    pub fn define_handler(&mut self, name: String, callback: Bound<PyAny>) -> PyResult<()> {
        check_callable(&callback)?;
        let callback_object = Py::from(callback);
        let callback_object_never_ends = &*Box::leak(Box::new(callback_object));
        self.teo_handler_group.define_handler(name.as_str(), move |request: TeoRequest| {
            async move {
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
            }
        });
        Ok(())
    }
}