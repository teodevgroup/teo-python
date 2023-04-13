use pyo3::{IntoPy, PyAny, PyObject, Python};
use pyo3_asyncio::{into_future_with_locals, TaskLocals};
use crate::utils::is_coroutine::is_coroutine;

pub fn await_coroutine_if_needed(transformed_py: &PyAny, py: Python<'_>) -> PyObject {
    let transformed_py = if is_coroutine(transformed_py, py)? {
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
            f.await
        })?
    } else {
        transformed_py.into_py(py)
    };
}