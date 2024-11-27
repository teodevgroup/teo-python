use key_path::Item;
use pyo3::{pyclass, pymethods, types::{PyList, PyListMethods}, IntoPy, PyObject, PyResult, Python};
use teo::prelude::pipeline;

use crate::{dynamic::py_class_lookup_map::PYClassLookupMap, object::{model::teo_model_object_to_py_any, value::teo_value_to_py_any}};

#[pyclass]
#[derive(Clone)]
pub struct PipelineCtx {
    pub(crate) ctx: pipeline::Ctx,
    pub(crate) map: &'static PYClassLookupMap,
}

impl From<pipeline::Ctx> for PipelineCtx {

    fn from(value: pipeline::Ctx) -> Self {
        let map = PYClassLookupMap::from_app_data(value.object().namespace().app_data());
        PipelineCtx {
            ctx: value,
            map
        }
    }
}

#[pymethods]
impl PipelineCtx {

    fn value(&self, py: Python<'_>) -> PyResult<PyObject> {
        teo_value_to_py_any(py, self.ctx.value(), self.map)
    }

    fn object(&self, py: Python<'_>) -> PyResult<PyObject> {
        teo_model_object_to_py_any(py, self.ctx.object(), self.map)
    }

    fn path(&self, py: Python<'_>) -> PyResult<PyObject> {
        let keypath = self.ctx.path();
        let list = PyList::empty_bound(py);
        for item in keypath {
            match item {
                Item::Index(n) => list.append(n)?,
                Item::Key(k) => list.append(k)?,
            }
        }
        Ok(list.into_py(py))
    }

    fn teo(&self, py: Python<'_>) -> PyResult<PyObject> {
        self.map.teo_transaction_ctx_to_py_ctx_object(py, self.ctx.transaction_ctx(), "")
    }
}