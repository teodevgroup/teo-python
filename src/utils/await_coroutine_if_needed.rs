use pyo3::{Py, Bound, IntoPy, PyAny, PyErr, PyObject, PyResult, Python};
use pyo3_async_runtimes::{into_future_with_locals, TaskLocals};
use crate::utils::is_coroutine::is_coroutine;

pub async fn await_coroutine_if_needed_value_with_locals<'py>(value: &Bound<'py, PyAny>, main_thread_locals: &TaskLocals) -> PyResult<Py<PyAny>> {
    let value_is_coroutine = is_coroutine(value)?;
    if value_is_coroutine {
        let f = Python::with_gil(|_py| {
            let f = into_future_with_locals(main_thread_locals, value.clone())?;
            Ok::<_, PyErr>(f)
        })?;
        let result = f.await?;
        Ok(result)
    } else {
        Python::with_gil(|py| {
            Ok(value.into_py(py))
        })
    }
}

pub async fn await_coroutine_if_needed(value: &Bound<'_, PyAny>) -> PyResult<PyObject> {
    if is_coroutine(value)? {
        let f = pyo3_async_runtimes::tokio::into_future(value.clone())?;
        f.await
    } else {
        Ok(Python::with_gil(|py| {
            value.into_py(py)
        }))
    }
}
