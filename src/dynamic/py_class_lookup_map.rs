use std::{collections::BTreeMap, ffi::CStr};

use pyo3::{exceptions::PyRuntimeError, types::{PyAnyMethods, PyCFunction, PyDict}, PyErr, PyObject, PyResult, Python};
use teo::prelude::{app::data::AppData, model, transaction};

use super::{model_object_wrapper::ModelObjectWrapper, transaction_ctx_wrapper::TransactionCtxWrapper};

static INIT_ERROR_MESSAGE_C: &CStr = c"class is not initialized";

static INIT_ERROR_MESSAGE: &str = "class is not initialized";

pub(crate) struct PYClassLookupMap {
    pub(crate) ctxs: BTreeMap<String, PyObject>,
    pub(crate) classes: BTreeMap<String, PyObject>,
    pub(crate) objects: BTreeMap<String, PyObject>,
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

    // Building methods

    pub(crate) fn insert_ctx(&mut self, name: &str, ctx: PyObject) {
        self.ctxs.insert(name.to_owned(), ctx);
    }

    pub(crate) fn insert_class(&mut self, name: &str, class: PyObject) {
        self.classes.insert(name.to_owned(), class);
    }

    pub(crate) fn insert_object(&mut self, name: &str, object: PyObject) {
        self.objects.insert(name.to_owned(), object);
    }

    pub(crate) fn ctx_or_create(&mut self, name: &str, py: Python<'_>) -> PyResult<PyObject> {
        let builtins = py.import("builtins")?;
        let py_type = builtins.getattr("type")?;
        let py_object = builtins.getattr("object")?;
        let dict = PyDict::new(py);
        dict.set_item("__module__", "teo.models")?;
        let init = PyCFunction::new_closure(py, Some(c"__init__"), None, |args, _kwargs| {
            let slf = args.get_item(0)?;
            let initialized: bool = slf.getattr("__teo_initialized__")?.extract()?;
            if initialized {
                Ok(())
            } else {
                Err::<(), PyErr>(PyRuntimeError::new_err(INIT_ERROR_MESSAGE))
            }
        })?;
        dict.set_item("__init__", init)?;
        let result = py_type.call1((name, (py_object,), dict))?.unbind();
        self.insert_ctx(name, result.clone_ref(py));
        Ok(result)
    }

    pub(crate) fn ctx(&self, name: &str) -> PyResult<Option<PyObject>> {
        Python::with_gil(|py| {
            Ok(self.ctxs.get(name).map(|o| o.clone_ref(py)))
        })
    }

    pub(crate) fn class_or_create(&mut self, name: &str, py: Python<'_>) -> PyResult<PyObject> {
        let builtins = py.import("builtins")?;
        let py_type = builtins.getattr("type")?;
        let py_object = builtins.getattr("object")?;
        let dict = PyDict::new(py);
        dict.set_item("__module__", "teo.models")?;
        let init = PyCFunction::new_closure(py, Some(c"__init__"), Some(INIT_ERROR_MESSAGE_C), |args, _kwargs| {
            let slf = args.get_item(0)?;
            let initialized: bool = slf.getattr("__teo_initialized__")?.extract()?;
            if initialized {
                Ok(())
            } else {
                Err::<(), PyErr>(PyRuntimeError::new_err(INIT_ERROR_MESSAGE))
            }
        })?;
        dict.set_item("__init__", init)?;
        let result = py_type.call1((name, (py_object,), dict))?.unbind();
        self.insert_class(name, result.clone_ref(py));
        Ok(result)
    }

    pub(crate) fn class(&self, name: &str) -> PyResult<Option<PyObject>> {
        Python::with_gil(|py| {
            Ok(self.classes.get(name).map(|o| o.clone_ref(py)))
        })
    }

    pub(crate) fn object_or_create(&mut self, name: &str, py: Python<'_>) -> PyResult<PyObject> {
        let builtins = py.import("builtins")?;
        let py_type = builtins.getattr("type")?;
        let py_object = builtins.getattr("object")?;
        let dict = PyDict::new(py);
        dict.set_item("__module__", "teo.models")?;
        let init = PyCFunction::new_closure(py, Some(c"__init__"), Some(INIT_ERROR_MESSAGE_C), |args, _kwargs| {
            let slf = args.get_item(0)?;
            let initialized: bool = slf.getattr("__teo_initialized__")?.extract()?;
            if initialized {
                Ok(())
            } else {
                Err::<(), PyErr>(PyRuntimeError::new_err(INIT_ERROR_MESSAGE))
            }
        })?;
        dict.set_item("__init__", init)?;
        let result = py_type.call1((name, (py_object,), dict))?.unbind();
        self.insert_object(name, result.clone_ref(py));
        Ok(result)
    }

    pub(crate) fn object(&self, name: &str) -> PyResult<Option<PyObject>> {
        Python::with_gil(|py| {
            Ok(self.objects.get(name).map(|o| o.clone_ref(py)))
        })
    }

    // Query methods

    pub(crate) fn teo_model_object_to_py_model_object_object(&self, py: Python<'_>, teo_model_object: model::Object) -> PyResult<PyObject> {
        let model_name = teo_model_object.model().path().join(".");
        let model_object_class = self.object(&model_name)?.unwrap();
        let model_object = model_object_class.call_method1(py, "__new__", (model_object_class.as_any(),))?;
        model_object.setattr(py, "__teo_object__", ModelObjectWrapper::new(teo_model_object))?;
        Ok(model_object)
    }

    pub(crate) fn teo_transaction_ctx_to_py_ctx_object(&self, py: Python<'_>, transaction_ctx: transaction::Ctx, name: &str) -> PyResult<PyObject> {
        let ctx_class = self.ctx(name)?.unwrap();
        let ctx_object = ctx_class.call_method1(py, "__new__", (ctx_class.as_any(),))?;
        ctx_object.setattr(py, "__teo_transaction_ctx__", TransactionCtxWrapper::new(transaction_ctx))?;
        Ok(ctx_object)
    }
}