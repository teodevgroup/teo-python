use pyo3::{pyclass, pymethods, types::{PyAnyMethods, PyDict, PyDictMethods}, IntoPy, PyResult, Python};
use teo::prelude::File as TeoFile;

/// File
/// File only represent input file in form request.
#[pyclass]
#[derive(Clone)]
pub struct File {
    #[pyo3(get, set)]
    pub filepath: String,
    #[pyo3(get, set)]
    pub content_type: Option<String>,
    #[pyo3(get, set)]
    pub filename: String,
    #[pyo3(get, set)]
    pub filename_ext: Option<String>,
}

#[pymethods]
impl File {

    pub fn __repr__(&self, py: Python<'_>) -> PyResult<String> {
        let prefix = "File(";
        let suffix = ")";
        let result = PyDict::new_bound(py);
        result.set_item("filepath", self.filepath.as_str())?;
        result.set_item("content_type", self.content_type.as_ref().into_py(py))?;
        result.set_item("filename", self.filename.as_str())?;
        result.set_item("filename_ext", self.content_type.as_ref().into_py(py))?;
        let dict_repr = result.call_method("__repr__", (), None)?;
        let dict_repr_str: &str = dict_repr.extract()?;
        Ok(format!("{}{}{}", prefix, dict_repr_str, suffix))
    }
}

impl From<&TeoFile> for File {
    fn from(value: &TeoFile) -> Self {
        Self {
            filepath: value.filepath.clone(),
            content_type: value.content_type.clone(),
            filename: value.filename.clone(),
            filename_ext: value.filename_ext.clone(),
        }
    }
}

impl From<&File> for TeoFile {
    fn from(value: &File) -> Self {
        Self {
            filepath: value.filepath.clone(),
            content_type: value.content_type.clone(),
            filename: value.filename.clone(),
            filename_ext: value.filename_ext.clone(),
        }
    }
}