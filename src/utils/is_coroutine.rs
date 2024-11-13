use pyo3::{Py, types::PyAnyMethods, PyAny, PyResult, Python};

pub fn is_coroutine(object: &Py<PyAny>) -> PyResult<bool> {
    Python::with_gil(|py| {
        let inspect = py.import_bound("inspect")?;
        let is_coroutine = inspect.getattr("iscoroutine")?;
        let result = is_coroutine.call1((object,))?;
        Ok(result.extract()?)    
    })
}