use std::{collections::BTreeMap, sync::Arc};
use pyo3::{exceptions::PyException, PyObject, PyResult};
use teo::prelude::app::data::AppData;
use super::{fetch::FetchDynamicClasses, query::QueryDynamicClasses};

#[derive(Clone)]
pub struct DynamicClasses {
    inner: Arc<Inner>,
}

struct Inner {
    ctxs: BTreeMap<String, PyObject>,
    classes: BTreeMap<String, PyObject>,
    objects: BTreeMap<String, PyObject>,
}

impl DynamicClasses {

    pub fn retrieve(app_data: &AppData) -> PyResult<Self> {
        let reference = app_data.dynamic_classes()?;
        let dynamic_classes: Option<&Self> = reference.downcast_ref();
        match dynamic_classes {
            Some(dynamic_classes) => Ok(dynamic_classes.clone()),
            None => Err(PyException::new_err("The dynamic classes attached on the app data is of wrong type")),
        }
    }

    pub fn attach(&self, app_data: AppData) -> PyResult<()> {
        Ok(app_data.set_dynamic_classes(Arc::new(self.clone()))?)
    }

    pub fn new(
        ctxs: BTreeMap<String, PyObject>, 
        classes: BTreeMap<String, PyObject>, 
        objects: BTreeMap<String, PyObject>
    ) -> Self {
        Self {
            inner: Arc::new(Inner {
                ctxs,
                classes,
                objects,    
            }),
        }
    }
}

impl FetchDynamicClasses for DynamicClasses {

    fn ctxs(&self) -> &BTreeMap<String, PyObject> {
        &self.inner.ctxs
    }

    fn classes(&self) -> &BTreeMap<String, PyObject> {
        &self.inner.classes
    }

    fn objects(&self) -> &BTreeMap<String, PyObject> {
        &self.inner.objects
    }
}

impl QueryDynamicClasses for DynamicClasses { }
