
use pyo3::{pyclass, IntoPyObjectExt, PyObject, PyResult, Python};
use teo::prelude::Pipeline as TeoPipeline;

#[pyclass]
pub struct Pipeline {
    pub(crate) value: TeoPipeline
}

pub fn teo_pipeline_to_py_any<'p>(py: Python<'p>, pipeline: &TeoPipeline) -> PyResult<PyObject> {
    Ok(Pipeline { value: pipeline.clone() }.into_py_any(py)?)
}