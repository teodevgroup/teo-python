pub mod convert;
pub mod utils;
pub mod result;
pub mod classes;
pub mod app;
pub mod namespace;
pub mod dynamic;
pub mod object;

use std::sync::Arc;
use pyo3::prelude::*;
use pyo3_asyncio::tokio::future_into_py;
use ::teo::prelude::{Value, App as TeoApp};
use pyo3::exceptions::PyRuntimeError;
use to_mut::ToMut;
use crate::classes::install::{generate_classes, get_model_class, setup_classes_container};
use crate::convert::to_py::teo_value_to_py_object;
use crate::convert::to_teo::py_object_to_teo_value;
use crate::result::IntoTeoResult;
use crate::utils::await_coroutine_if_needed::await_coroutine_if_needed;
use crate::utils::check_callable::check_callable;
use crate::utils::is_coroutine::is_coroutine;
use crate::utils::validate_result::validate_result;

use crate::app::app::App;
use crate::namespace::namespace::Namespace;


#[pymodule]
fn teo(_py: Python, m: &PyModule) -> PyResult<()> {
    setup_classes_container()?;
    #[pyfunction]
    fn fetch_model_class(name: &str, py: Python) -> PyResult<PyObject> {
        get_model_class(name, py)
    }
    m.add_function(wrap_pyfunction!(fetch_model_class, m)?)?;
    m.add_class::<App>()?;
    m.add_class::<Namespace>()?;
    Ok(())
}
