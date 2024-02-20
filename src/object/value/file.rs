use pyo3::{pyclass, pymethods};
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

    // pub fn __repr__(&self) -> PyResult<String> {
    //     Python::with_gil(|py| {
    //         let slf = args.get_item(0)?.into_py(py);
    //         let model_object_wrapper: ModelObjectWrapper = slf.getattr(py, "__teo_object__")?.extract(py)?;
    //         let result = PyDict::new(py);
    //         let value_map = model_object_wrapper.object.inner.value_map.lock().unwrap();
    //         for (k, v) in value_map.iter() {
    //             result.set_item(k, teo_value_to_py_any(py, v)?)?;
    //         }
    //         let dict_repr = result.call_method("__repr__", (), None)?;
    //         let dict_repr_str: &str = dict_repr.extract()?;
    //         let prefix = format!("{}(", model_object_wrapper.object.model().path().join("."));
    //         let suffix = ")";
    //         Ok::<PyObject, PyErr>(format!("{}{}{}", prefix, dict_repr_str, suffix).into_py(py))
    //     })
    // }
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