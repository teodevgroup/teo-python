use indexmap::IndexMap;
use pyo3::{exceptions::PyRuntimeError, import_exception, types::{PyList, PyType}, IntoPy, PyErr, PyObject, PyResult, Python};

import_exception!(teo, TeoException);

pub trait IntoTeoResult<T> {
    fn into_teo_result(self) -> ::teo::prelude::Result<T>;
}

impl<T> IntoTeoResult<T> for PyResult<T> {
    fn into_teo_result(self) -> teo::prelude::Result<T> {
        Python::with_gil(|py| {
            match self {
                Ok(r) => Ok(r),
                Err(e) => {
                    if e.get_type(py).is(PyType::new::<TeoException>(py)) {
                        let py_object: PyObject = e.clone_ref(py).into_py(py);
                        let message: String = py_object.getattr(py, "message").into_teo_result()?.extract(py).into_teo_result()?;
                        let code: Option<u16> = py_object.getattr(py, "code").into_teo_result()?.extract(py).into_teo_result()?;
                        let title: Option<String> = py_object.getattr(py, "title").into_teo_result()?.extract(py).into_teo_result()?;
                        let prefixes: Option<Vec<String>> = py_object.getattr(py, "prefixes").into_teo_result()?.extract(py).into_teo_result()?;
                        let fields: Option<IndexMap<String, String>> = py_object.getattr(py, "fields").into_teo_result()?.extract(py).into_teo_result()?;
                        let mut error = ::teo::prelude::Error::new(message);
                        error.code = code;
                        error.title = title;
                        error.prefixes = prefixes;
                        error.fields = fields;
                        error.assign_platform_native_object(e);
                        Err(error)
                    } else {
                        let mut error = ::teo::prelude::Error::new(e.to_string());
                        error.assign_platform_native_object(e);
                        Err(error)
                    }
                },
            }    
        })
    }
}

pub trait IntoPyResult<T> {
    fn into_py_result(self, py: Python<'_>) -> PyResult<T>;
}

impl<T> IntoPyResult<T> for teo::prelude::Result<T> {
    fn into_py_result(self, py: Python<'_>) -> PyResult<T> {
        match self {
            Ok(r) => Ok(r),
            Err(e) => {
                let meta: Option<&PyErr> = e.platform_native_object();
                if let Some(err) = meta {
                    Err(PyErr::from_value(err.into_py(py).as_ref(py)))
                } else {
                    
                    Err(PyRuntimeError::new_err(e.message().to_owned()))
                }
            },
        }
    }
}

pub trait IntoPyResultWithGil<T> {
    fn into_py_result_with_gil(self) -> PyResult<T>;
}

impl<T> IntoPyResultWithGil<T> for teo::prelude::Result<T> {
    fn into_py_result_with_gil(self) -> PyResult<T> {
        Python::with_gil(|py| {
            self.into_py_result(py)
        })
    }
}
