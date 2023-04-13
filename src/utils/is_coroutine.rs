use pyo3::{PyAny, PyResult, Python};

pub fn is_coroutine(object: &PyAny, py: Python<'_>) -> PyResult<bool> {
    let inspect = py.import("inspect")?;
    let is_coroutine = inspect.getattr("iscoroutine")?;
    let result = is_coroutine.call1((object,))?;
    Ok(result.extract()?)
}