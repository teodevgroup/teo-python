use std::collections::BTreeMap;
use pyo3::PyObject;
use super::{CreateDynamicClasses, DynamicClasses, FetchDynamicClasses, QueryDynamicClasses};

pub struct DynamicClassesBuilder {
    ctxs: BTreeMap<String, PyObject>,
    classes: BTreeMap<String, PyObject>,
    objects: BTreeMap<String, PyObject>,
}

impl DynamicClassesBuilder {

    pub fn new() -> Self {
        Self {
            ctxs: BTreeMap::new(),
            classes: BTreeMap::new(),
            objects: BTreeMap::new(),
        }
    }

    pub fn build(self) -> DynamicClasses {
        DynamicClasses::new(self.ctxs, self.classes, self.objects)
    }
}

impl FetchDynamicClasses for DynamicClassesBuilder {
    
    fn ctxs(&self) -> &BTreeMap<String, PyObject> {
        &self.ctxs
    }

    fn classes(&self) -> &BTreeMap<String, PyObject> {
        &self.classes
    }

    fn objects(&self) -> &BTreeMap<String, PyObject> {
        &self.objects
    }
}

impl CreateDynamicClasses for DynamicClassesBuilder {

    fn ctxs_mut(&mut self) -> &mut BTreeMap<String, PyObject> {
        &mut self.ctxs
    }

    fn classes_mut(&mut self) -> &mut BTreeMap<String, PyObject> {
        &mut self.classes
    }

    fn objects_mut(&mut self) -> &mut BTreeMap<String, PyObject> {
        &mut self.objects
    }
}

impl QueryDynamicClasses for DynamicClassesBuilder { }
