use key_path::Item;
use pyo3::{pyclass, pymethods, types::{PyList, PyListMethods}, PyObject, PyResult, Python};
use teo::prelude::pipeline;

use crate::{dynamic::py_class_lookup_map::PYClassLookupMap, object::{model::teo_model_object_to_py_any, value::teo_value_to_py_any}, request::Request};

#[pyclass]
#[derive(Clone)]
pub struct PipelineCtx {
    pub(crate) original: pipeline::Ctx,
    pub(crate) map: &'static PYClassLookupMap,
}

impl From<pipeline::Ctx> for PipelineCtx {

    fn from(original: pipeline::Ctx) -> Self {
        let map = PYClassLookupMap::from_app_data(original.object().namespace().app_data());
        Self {
            original,
            map
        }
    }
}

#[pymethods]
impl PipelineCtx {

    #[getter]
    fn value(&self, py: Python<'_>) -> PyResult<PyObject> {
        teo_value_to_py_any(py, self.original.value(), self.map)
    }

    #[getter]
    fn object(&self, py: Python<'_>) -> PyResult<PyObject> {
        teo_model_object_to_py_any(py, self.original.object(), self.map)
    }

    #[getter]
    fn path(&self, py: Python<'_>) -> PyResult<PyObject> {
        let keypath = self.original.path();
        let list = PyList::empty(py);
        for item in keypath {
            match item {
                Item::Index(n) => list.append(n)?,
                Item::Key(k) => list.append(k)?,
            }
        }
        Ok(list.into_any().unbind())
    }

    #[getter]
    fn teo(&self, py: Python<'_>) -> PyResult<PyObject> {
        self.map.teo_transaction_ctx_to_py_ctx_object(py, self.original.transaction_ctx(), "")
    }

    #[getter]
    fn request(&self) -> Option<Request> {
        self.original.request().map(|r| Request::from(r))
    }
}