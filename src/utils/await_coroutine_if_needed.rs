use pyo3::{IntoPy, PyAny, PyErr, PyObject, PyResult, Python};
use pyo3_asyncio::{into_future_with_locals, TaskLocals};
use crate::utils::is_coroutine::is_coroutine;

fn check_event_loop() -> PyResult<Option<PyObject>> {
    Python::with_gil(|py| {
        let asyncio = py.import("asyncio")?;
        let event_loop = asyncio.call_method0("get_event_loop");
        if event_loop.is_ok() {
            return Ok(None)
        } else {
            let event_loop = asyncio.call_method0("new_event_loop")?;
            asyncio.call_method1("set_event_loop", (event_loop,))?;
            return Ok(Some(event_loop.into_py(py)))
        }
    })
}

pub async fn await_coroutine_if_needed_async_value(transformed_py: PyObject) -> PyResult<PyObject> {
    let is_coroutine_bool = Python::with_gil(|py| {
        is_coroutine(transformed_py.as_ref(py))
    })?;
    if is_coroutine_bool {
        if let Some(event_loop) = check_event_loop()? {
            Python::with_gil(|py| {
                let f = into_future_with_locals(&TaskLocals::new(event_loop.as_ref(py)), transformed_py.as_ref(py))?;
                pyo3_asyncio::tokio::run_until_complete(event_loop.as_ref(py), async move {
                    let result = f.await;
                    result
                })
            })
        } else {
            let f = Python::with_gil(|py| {
                let event_loop = pyo3_asyncio::tokio::get_current_loop(py)?;
                let f = into_future_with_locals(&TaskLocals::new(event_loop), transformed_py.as_ref(py))?;
                Ok::<_, PyErr>(f)
            })?;
            f.await    
        }
    } else {
        Ok(Python::with_gil(|py| {
            transformed_py.into_py(py)
        }))
    }
}

pub async fn await_coroutine_if_needed_async(transformed_py: &PyAny) -> PyResult<PyObject> {
    if is_coroutine(transformed_py)? {
        let f = pyo3_asyncio::tokio::into_future(transformed_py)?;
        f.await
    } else {
        Ok(Python::with_gil(|py| {
            transformed_py.into_py(py)
        }))
    }
}

pub fn await_coroutine_if_needed(py: Python<'_>, transformed_py: &PyAny) -> PyResult<PyObject> {
    Ok(if is_coroutine(transformed_py)? {
        println!("is coroutine");
        let asyncio = py.import("asyncio")?;
        let event_loop = asyncio.call_method0("get_event_loop").unwrap_or(
            {
                let event_loop = asyncio.call_method0("new_event_loop")?;
                asyncio.call_method1("set_event_loop", (event_loop,))?;
                event_loop
            }
        );
        let f = into_future_with_locals(&TaskLocals::new(event_loop), transformed_py)?;
        pyo3_asyncio::tokio::run_until_complete(event_loop, async move {
            println!("run until complete");
            let result = f.await;
            println!("run until complete done");
            result
        })?
    } else {
        println!("is not coroutine");
        transformed_py.into_py(py)
    })
}