use pyo3::{PyAny, PyResult, Python};

pub fn is_coroutine(object: &PyAny) -> PyResult<bool> {
    Python::with_gil(|py| {
        let inspect = py.import("inspect")?;
        let is_coroutine = inspect.getattr("iscoroutine")?;
        let result = is_coroutine.call1((object,))?;
        Ok(result.extract()?)    
    })
}