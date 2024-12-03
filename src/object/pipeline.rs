
use pyo3::{pyclass, pymethods, IntoPyObjectExt, PyObject, PyResult, Python};
use teo::prelude::Pipeline as OriginalPipeline;

#[pyclass]
pub struct Pipeline {
    pub(crate) original: OriginalPipeline
}

#[pymethods]
impl Pipeline {

    pub fn __len__(&self) -> usize {
        self.original.len()
    }

    pub fn __str__(&self) -> String {
        format!("{}", self.original)
    }

    pub fn __repr__(&self) -> String {
        format!("{}", self.original)
    }
}

pub fn teo_pipeline_to_py_any<'p>(py: Python<'p>, pipeline: &OriginalPipeline) -> PyResult<PyObject> {
    Ok(Pipeline { original: pipeline.clone() }.into_py_any(py)?)
}