use std::collections::BTreeMap;

use pyo3::{exceptions::PyRuntimeError, types::{PyAnyMethods, PyCFunction, PyDict}, IntoPy, PyErr, PyObject, PyResult, Python};
use teo::prelude::{app::data::AppData, model, transaction};

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

    pub(crate) fn ctxs(&self) -> &BTreeMap<String, PyObject> {
        &self.ctxs
    }

    pub(crate) fn classes(&self) -> &BTreeMap<String, PyObject> {
        &self.classes
    }

    pub(crate) fn objects(&self) -> &BTreeMap<String, PyObject> {
        &self.objects
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
        let builtins = py.import_bound("builtins")?;
        let py_type = builtins.getattr("type")?;
        let py_object = builtins.getattr("object")?;
        let dict = PyDict::new_bound(py);
        dict.set_item("__module__", "teo.models")?;
        let init = PyCFunction::new_closure_bound(py, Some("__init__"), None, |args, _kwargs| {
            let slf = args.get_item(0)?;
            let initialized: bool = slf.getattr("__teo_initialized__")?.extract()?;
            if initialized {
                Ok(())
            } else {
                Err::<(), PyErr>(PyRuntimeError::new_err(INIT_ERROR_MESSAGE))
            }
        })?;
        dict.set_item("__init__", init)?;
        let result = py_type.call1((name, (py_object,), dict))?;
        let result_object = result.clone().into_py(py);
        self.insert_ctx(name, result_object);
        Ok(result.into_py(py))
    }

    pub(crate) fn ctx(&self, name: &str) -> PyResult<Option<PyObject>> {
        Ok(self.ctxs.get(name).cloned())
    }

    pub(crate) fn class_or_create(&mut self, name: &str, py: Python<'_>) -> PyResult<PyObject> {
        let builtins = py.import("builtins")?;
        let py_type = builtins.getattr("type")?;
        let py_object = builtins.getattr("object")?;
        let dict = PyDict::new(py);
        dict.set_item("__module__", "teo.models")?;
        let init = PyCFunction::new_closure_bound(py, Some("__init__"), Some(INIT_ERROR_MESSAGE), |args, _kwargs| {
            let slf = args.get_item(0)?;
            let initialized: bool = slf.getattr("__teo_initialized__")?.extract()?;
            if initialized {
                Ok(())
            } else {
                Err::<(), PyErr>(PyRuntimeError::new_err(INIT_ERROR_MESSAGE))
            }
        })?;
        dict.set_item("__init__", init)?;
        let result = py_type.call1((name, (py_object,), dict))?;
        let result_object = result.into_py(py);
        self.insert_class(name, result_object);
        Ok(result.into_py(py))
    }

    pub(crate) fn class(&self, name: &str) -> PyResult<Option<PyObject>> {
        Ok(self.classes.get(name).cloned())
    }

    pub(crate) fn object_or_create(&mut self, name: &str, py: Python<'_>) -> PyResult<PyObject> {
        let builtins = py.import("builtins")?;
        let py_type = builtins.getattr("type")?;
        let py_object = builtins.getattr("object")?;
        let dict = PyDict::new(py);
        dict.set_item("__module__", "teo.models")?;
        let init = PyCFunction::new_closure_bound(py, Some("__init__"), Some(INIT_ERROR_MESSAGE), |args, _kwargs| {
            let slf = args.get_item(0)?;
            let initialized: bool = slf.getattr("__teo_initialized__")?.extract()?;
            if initialized {
                Ok(())
            } else {
                Err::<(), PyErr>(PyRuntimeError::new_err(INIT_ERROR_MESSAGE))
            }
        })?;
        dict.set_item("__init__", init)?;
        let result = py_type.call1((name, (py_object,), dict))?;
        let result_object = result.into_py(py);
        self.insert_object(name, result_object);
        Ok(result.into_py(py))
    }

    pub(crate) fn object(&self, name: &str) -> PyResult<Option<PyObject>> {
        Ok(self.objects.get(name).cloned())
    }

    // Query methods

    pub(crate) fn teo_model_ctx_to_py_model_class_object(&self, py: Python<'_>, model_ctx: model::Ctx, name: &str) -> PyResult<PyObject> {
        let Some(class_prototype) = self.class_prototype(name, env)? else {
            return Err(Error::from_reason("Class prototype not found"));
        };
        let mut js_object = env.create_object()?;
        js_object.set_named_property("__proto__", class_prototype)?;
        env.wrap(&mut js_object, model_ctx)?;
        Ok(js_object)
    }

    pub(crate) fn teo_model_object_to_py_model_object_object(&self, py: Python<'_>, teo_model_object: model::Object) -> PyResult<PyObject> {
        let Some(object_prototype) = self.object_prototype(&teo_model_object.model().path().join("."), env)? else {
            return Err(Error::from_reason("Object prototype not found"));
        };
        let mut js_object = env.create_object()?;
        js_object.set_named_property("__proto__", object_prototype)?;
        env.wrap(&mut js_object, teo_model_object)?;
        Ok(js_object)
    }

    pub(crate) fn teo_optional_model_object_to_py_optional_model_object_object(&self, py: Python<'_>, teo_model_object: Option<model::Object>) -> PyResult<PyObject> {
        Ok(match teo_model_object {
            Some(teo_model_object) => self.teo_model_object_to_js_model_object_object(env, teo_model_object)?.into_unknown(),
            None => env.get_undefined()?.into_unknown(),
        })
    }

    pub(crate) fn teo_transaction_ctx_to_py_ctx_object(&self, py: Python<'_>, transaction_ctx: transaction::Ctx, name: &str) -> PyResult<PyObject> {
        let Some(ctx_prototype) = self.ctx_prototype(name, env)? else {
            return Err(Error::from_reason("Ctx prototype not found"));
        };
        let mut js_object = env.create_object()?;
        js_object.set_named_property("__proto__", ctx_prototype)?;
        env.wrap(&mut js_object, transaction_ctx)?;
        Ok(js_object)
    }
}