use pyo3::{exceptions::PyRuntimeError, PyResult, PyErr, IntoPy, Python};

pub trait IntoTeoResult<T> {
    fn into_teo_result(self) -> ::teo::prelude::Result<T>;
}

impl<T> IntoTeoResult<T> for PyResult<T> {
    fn into_teo_result(self) -> teo::prelude::Result<T> {
        match self {
            Ok(r) => Ok(r),
            Err(e) => {
                let mut error = ::teo::prelude::Error::new(e.to_string());
                error.assign_platform_native_object(e);
                Err(error)
            },
        }
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
