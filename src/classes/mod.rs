use std::collections::HashMap;
use std::ffi::{c_char, c_int, c_void, CString};
use std::mem;
use std::ptr::null_mut;
use ::teo::prelude::App;
use pyo3::{AsPyPointer, FromPyPointer, IntoPy, PyAny, PyErr, PyObject, PyResult, Python};
use pyo3::ffi::{Py_tp_init, PyErr_SetString, PyExc_RuntimeError, PyType_FromModuleAndSpec, PyType_Slot, PyType_Spec};
use pyo3::types::{PyModule};
use teo::prelude::Object;

static mut CLASSES: Option<&'static HashMap<String, PyObject>> = None;
static mut MODULE: Option<&'static PyObject> = None;

fn classes_mut() -> &'static mut HashMap<String, PyObject> {
    unsafe {
        let const_ptr = CLASSES.unwrap() as *const HashMap<String, PyObject>;
        let mut_ptr = const_ptr as *mut HashMap<String, PyObject>;
        &mut *mut_ptr
    }
}

fn get_model_module() -> &'static PyObject {
    unsafe {
        MODULE.unwrap()
    }
}

pub fn get_model_class(name: &str, py: Python<'_>) -> PyResult<PyObject> {
    unsafe {
        if let Some(object_ref) = CLASSES.unwrap().get(name) {
            Ok(object_ref.clone_ref(py))
        } else {
            generate_model_class(name, py)
        }
    }
}

const CLASS_TP_INIT_ERROR_MSG: &'static [u8] = b"Do not call Teo model init directly. Use `create' instead.\0";

#[no_mangle]
unsafe extern "C" fn class_tp_init(_this: &mut pyo3::ffi::PyObject, _args: &mut pyo3::ffi::PyObject, _kwargs: &mut pyo3::ffi::PyObject) -> c_int {
    PyErr_SetString(PyExc_RuntimeError, CLASS_TP_INIT_ERROR_MSG.as_ptr() as *const c_char);
    -1
}

unsafe fn generate_model_class(name: &str, py: Python<'_>) -> PyResult<PyObject> {
    let slots = Box::into_raw(Box::new([
        PyType_Slot { slot: Py_tp_init, pfunc: class_tp_init as *mut c_void },
        PyType_Slot { slot: 0, pfunc: null_mut() },
    ]));
    let mut spec = PyType_Spec {
        name: {
            let c_string = Box::leak(Box::new(CString::new("teo.models.".to_string() + name).unwrap()));
            c_string.as_ptr()
        },
        basicsize: mem::size_of::<Object>() as c_int,
        itemsize: 0,
        flags: 0,
        slots: slots as *mut PyType_Slot,
    };
    let module = get_model_module().as_ptr();
    let t = PyType_FromModuleAndSpec(module, &mut spec, null_mut());
    let result = PyAny::from_owned_ptr_or_err(py, t)?;
    result.setattr("a", 1)?;
    Ok(result.into_py(py))
}

pub fn setup_classes_container() -> PyResult<()> {
    unsafe {
        Python::with_gil(|py| {
            MODULE = Some(Box::leak(Box::new(PyModule::new(py, "teo.model")?.into_py(py))));
            Ok::<(), PyErr>(())
        })?;
        CLASSES = Some(Box::leak(Box::new(HashMap::new())))
    };
    Ok(())
}

pub fn generate_classes(app: &App, _py: Python<'_>) -> PyResult<()> {
    for _model in app.graph().models() {

    }
    Ok(())
}