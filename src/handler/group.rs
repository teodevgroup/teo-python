use pyo3::types::{PyAnyMethods, PyNone};
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

    pub fn define_handler(&mut self, name: String, callback: Bound<PyAny>, py: Python<'_>) -> PyResult<()> {
        check_callable(&callback)?;
        let main_thread_locals = &*Box::leak(Box::new(pyo3_async_runtimes::tokio::get_current_locals(py)?));
        let callback_object = Py::from(callback);
        let callback_object_never_ends = &*Box::leak(Box::new(callback_object));
        self.teo_handler_group.define_handler(name.as_str(), move |request: TeoRequest| {
            async move {
                let result = Python::with_gil(|py| {
                    let request = Request::new(request);
                    let result = callback_object_never_ends.call1(py, (request,))?;
                    let current_thread_locals_result = pyo3_async_runtimes::tokio::get_current_locals(py);
                    if let Ok(current_thread_locals) = current_thread_locals_result {
                        Ok::<(PyObject, Option<TaskLocals>, Option<&TaskLocals>), PyErr>((result, Some(current_thread_locals), None))
                    } else {
                        Ok::<(PyObject, Option<TaskLocals>, Option<&TaskLocals>), PyErr>((result, None, Some(main_thread_locals)))
                    }
                })?;
                let locals = if result.2.is_some() {
                    result.2.unwrap()
                } else {
                    result.1.as_ref().unwrap()
                };
                let awaited_result = await_coroutine_if_needed_value_with_locals(&result.0, locals).await?;
                Python::with_gil(|py| {
                    let response: Response = awaited_result.extract(py)?;
                    Ok(response.teo_response.clone())
                })
            }
        });
        Ok(())
    }
}