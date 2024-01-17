use pyo3::{exceptions::PyRuntimeError, PyResult, PyErr};

pub trait IntoTeoResult<T> {
    fn into_teo_result(self) -> ::teo::prelude::Result<T>;
}

impl<T> IntoTeoResult<T> for PyResult<T> {
    fn into_teo_result(self) -> teo::prelude::Result<T> {
        match self {
            Ok(r) => Ok(r),
            Err(e) => {
                let mut e = ::teo::prelude::Error::new(e.to_string());
                e.insert_meta("pyErr", e);
                Err(e)
            },
        }
    }
}

pub trait IntoPyResult<T> {
    fn into_py_result(self) -> PyResult<T>;
}

impl<T> IntoPyResult<T> for teo::prelude::Result<T> {
    fn into_py_result(self) -> PyResult<T> {
        match self {
            Ok(r) => Ok(r),
            Err(e) => {
                let meta: Option<&PyErr> = e.get_meta("pyErr");
                if let Some(err) = meta {
                    Err(err.clone())
                } else {
                    Err(PyRuntimeError::new_err(e.message().to_owned()))
                }
            },
        }
    }
}