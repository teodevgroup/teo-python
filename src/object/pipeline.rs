
use pyo3::{pyclass, PyResult, Python, PyAny, IntoPy};
use teo::prelude::Pipeline as TeoPipeline;

#[pyclass]
pub struct Pipeline {
    pub(crate) value: TeoPipeline
}

pub fn teo_pipeline_to_py_any<'p>(py: Python<'p>, pipeline: &TeoPipeline) -> PyResult<PyAny> {
    let instance = Pipeline { value: pipeline.clone() }.into_py(py)?;
    Ok(instance)
}