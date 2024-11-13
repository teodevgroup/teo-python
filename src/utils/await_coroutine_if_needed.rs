use pyo3::{Py, IntoPy, PyAny, PyErr, PyObject, PyResult, Python};
use pyo3_async_runtimes::{into_future_with_locals, TaskLocals};
use crate::utils::is_coroutine::is_coroutine;

pub async fn await_coroutine_if_needed_value_with_locals<'py>(value: &Py<PyAny>, main_thread_locals: &TaskLocals) -> PyResult<Py<PyAny>> {
    let value_is_coroutine = is_coroutine(value)?;
    if value_is_coroutine {
        let f = Python::with_gil(|py| {
            let f = into_future_with_locals(main_thread_locals, value.into_py(py).into_bound(py))?;
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

pub async fn await_coroutine_if_needed(value: &Py<PyAny>) -> PyResult<PyObject> {
    if is_coroutine(value)? {
        let f = Python::with_gil(|py| {
            pyo3_async_runtimes::tokio::into_future(value.into_py(py).into_bound(py))
        })?;
        Ok(f.await?)
    } else {
        Ok(Python::with_gil(|py| {
            value.into_py(py)
        }))
    }
}
