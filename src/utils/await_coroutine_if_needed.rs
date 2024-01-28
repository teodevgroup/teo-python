use pyo3::{IntoPy, PyAny, PyErr, PyObject, PyResult, Python};
use pyo3_asyncio::{into_future_with_locals, TaskLocals};
use crate::utils::is_coroutine::is_coroutine;

pub async fn await_coroutine_if_needed_value_with_locals(transformed_py: PyObject, main_thread_locals: &TaskLocals) -> PyResult<PyObject> {
    let is_coroutine_bool = Python::with_gil(|py| {
        is_coroutine(transformed_py.as_ref(py))
    })?;
    if is_coroutine_bool {
        let f = Python::with_gil(|py| {
            let f = into_future_with_locals(main_thread_locals, transformed_py.as_ref(py))?;
            Ok::<_, PyErr>(f)
        })?;
        f.await
    } else {
        Ok(Python::with_gil(|py| {
            transformed_py.into_py(py)
        }))
    }
}

pub async fn await_coroutine_if_needed(transformed_py: &PyAny) -> PyResult<PyObject> {
    if is_coroutine(transformed_py)? {
        let f = pyo3_asyncio::tokio::into_future(transformed_py)?;
        f.await
    } else {
        Ok(Python::with_gil(|py| {
            transformed_py.into_py(py)
        }))
    }
}
