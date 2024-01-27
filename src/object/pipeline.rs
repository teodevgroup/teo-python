
use pyo3::{pyclass, PyResult, Python, IntoPy, PyObject};
use teo::prelude::Pipeline as TeoPipeline;

#[pyclass]
pub struct Pipeline {
    pub(crate) value: TeoPipeline
}

pub fn teo_pipeline_to_py_any<'p>(py: Python<'p>, pipeline: &TeoPipeline) -> PyResult<PyObject> {
    let instance = Pipeline { value: pipeline.clone() }.into_py(py);
    Ok(instance)
}