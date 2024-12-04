use pyo3::{pyclass, pymethods, Py, PyErr, PyObject, PyResult, Python};
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

    pub fn _define_handler(&mut self, name: String, callback: PyObject, py: Python<'_>) -> PyResult<()> {
        check_callable(callback.bind(py))?;
        let main_thread_locals = pyo3_async_runtimes::tokio::get_current_locals(py)?;
        let callback_object = Py::from(callback);
        self.teo_handler_group.define_handler(name.as_str(), move |request: TeoRequest| {
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
}