pub mod utils;
pub mod result;
pub mod app;
pub mod namespace;
pub mod dynamic;
pub mod object;
pub mod r#enum;

use std::sync::Arc;
use pyo3::prelude::*;
use pyo3_asyncio::tokio::future_into_py;
use ::teo::prelude::{Value, App as TeoApp};
use pyo3::exceptions::PyRuntimeError;
use crate::classes::install::{generate_classes, get_model_class, setup_classes_container};
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
