use std::collections::HashMap;
use std::ffi::{c_char, c_int, c_void, CStr, CString};
use std::mem;
use std::ptr::null_mut;
use ::teo::prelude::App;
use pyo3::{FromPyPointer, IntoPy, PyAny, PyObject, PyResult, Python};
use pyo3::ffi::{Py_tp_init, PyBool_Type, PyErr_SetString, PyExc_Exception, PyExc_RuntimeError, PyObject_Type, PyType_FromSpec, PyType_GenericNew, PyType_Slot, PyType_Spec, PyTypeObject};
use pyo3::types::PyType;
use teo::prelude::Object;

static mut CLASSES: Option<&'static HashMap<String, PyObject>> = None;

fn classes_mut() -> &'static mut HashMap<String, PyObject> {
    unsafe {
        let const_ptr = CLASSES.unwrap() as *const HashMap<String, PyObject>;
        let mut_ptr = const_ptr as *mut HashMap<String, PyObject>;
        &mut *mut_ptr
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
unsafe extern "C" fn class_tp_init(this: &mut pyo3::ffi::PyObject, args: &mut pyo3::ffi::PyObject, kwargs: &mut pyo3::ffi::PyObject) -> c_int {
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
            let c_string = Box::leak(Box::new(CString::new(name).unwrap()));
            c_string.as_ptr()
        },
        basicsize: mem::size_of::<Object>() as c_int,
        itemsize: 0,
        flags: 0,
        slots: slots as *mut PyType_Slot,
    };
    let t = PyType_FromSpec(&mut spec);
    let result = PyAny::from_owned_ptr_or_err(py, t)?;
    result.setattr("a", 1)?;
    Ok(result.into_py(py))
}

pub fn setup_classes_container() {
    unsafe { CLASSES = Some(Box::leak(Box::new(HashMap::new()))) };
}

pub fn generate_classes(app: &App, py: Python<'_>) -> PyResult<()> {
    for model in app.graph().models() {

    }
    Ok(())
}