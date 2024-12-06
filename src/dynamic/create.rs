use std::{collections::BTreeMap, ffi::CStr};
use pyo3::{exceptions::PyRuntimeError, types::{PyAnyMethods, PyCFunction, PyDict}, PyErr, PyObject, PyResult, Python};
use super::fetch::FetchDynamicClasses;

static INIT_ERROR_MESSAGE_C: &CStr = c"class is not initialized";
static INIT_ERROR_MESSAGE: &str = "class is not initialized";

pub trait CreateDynamicClasses: FetchDynamicClasses {

    fn ctxs_mut(&mut self) -> &mut BTreeMap<String, PyObject>;

    fn classes_mut(&mut self) -> &mut BTreeMap<String, PyObject>;

    fn objects_mut(&mut self) -> &mut BTreeMap<String, PyObject>;

    fn ctx_or_create(&mut self, name: &str, py: Python<'_>) -> PyResult<PyObject> {
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
        self.ctxs_mut().insert(name.to_owned(), result.clone_ref(py));
        Ok(result)
    }

    fn class_or_create(&mut self, name: &str, py: Python<'_>) -> PyResult<PyObject> {
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
        self.classes_mut().insert(name.to_owned(), result.clone_ref(py));
        Ok(result)
    }

    fn object_or_create(&mut self, name: &str, py: Python<'_>) -> PyResult<PyObject> {
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
        self.objects_mut().insert(name.to_owned(), result.clone_ref(py));
        Ok(result)
    }
}
