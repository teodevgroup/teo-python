use pyo3::{types::PyAnyMethods, Bound, PyAny, PyResult, Python};

pub fn is_coroutine(object: &Bound<PyAny>) -> PyResult<bool> {
    Python::with_gil(|py| {
        let inspect = py.import_bound("inspect")?;
        let is_coroutine = inspect.getattr("iscoroutine")?;
        let result = is_coroutine.call1((object,))?;
        Ok(result.extract()?)    
    })
}