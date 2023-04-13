use pyo3::PyResult;

pub trait IntoTeoResult<T> {
    fn into_teo_result(self) -> ::teo::prelude::Result<T>;
}

impl<T> IntoTeoResult<T> for PyResult<T> {
    fn into_teo_result(self) -> teo::prelude::Result<T> {
        match self {
            Ok(r) => Ok(r),
            Err(e) => Err(::teo::prelude::Error::custom_internal_server_error(e.to_string())),
        }
    }
}
