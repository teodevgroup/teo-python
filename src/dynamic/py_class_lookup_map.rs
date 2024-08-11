use std::collections::BTreeMap;

use teo::prelude::app::data::AppData;

use super::{model_ctx_wrapper::ModelCtxWrapper, model_object_wrapper::ModelObjectWrapper, transaction_ctx_wrapper::TransactionCtxWrapper};

pub(crate) struct PYClassLookupMap {
    pub(crate) ctxs: BTreeMap<String, TransactionCtxWrapper>,
    pub(crate) classes: BTreeMap<String, ModelCtxWrapper>,
    pub(crate) objects: BTreeMap<String, ModelObjectWrapper>,
}

impl PYClassLookupMap {

    pub(crate) fn from_app_data(app_data: &AppData) -> &'static Self {
        unsafe {
            let pointer: *mut PYClassLookupMap = app_data.dynamic_classes_pointer() as * mut Self;
            &*pointer as &PYClassLookupMap
        }
    }

    pub(crate) fn new() -> Self {
        Self {
            ctxs: BTreeMap::new(),
            classes: BTreeMap::new(),
            objects: BTreeMap::new(),
        }
    }

    pub(crate) fn ctxs(&self) -> &BTreeMap<String, TransactionCtxWrapper> {
        &self.ctxs
    }

    pub(crate) fn classes(&self) -> &BTreeMap<String, ModelCtxWrapper> {
        &self.classes
    }

    pub(crate) fn objects(&self) -> &BTreeMap<String, ModelObjectWrapper> {
        &self.objects
    }

    // Building methods

    pub(crate) fn insert_ctx(&mut self, name: String, ctx: TransactionCtxWrapper) {
        self.ctxs.insert(name, ctx);
    }

    pub(crate) fn insert_class(&mut self, name: String, class: ModelCtxWrapper) {
        self.classes.insert(name, class);
    }

    pub(crate) fn insert_object(&mut self, name: String, object: ModelObjectWrapper) {
        self.objects.insert(name, object);
    }


}