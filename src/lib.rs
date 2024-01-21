pub mod utils;
pub mod result;
pub mod app;
pub mod namespace;
pub mod dynamic;
pub mod object;
pub mod r#enum;
pub mod model;
pub mod handler;
pub mod request;
pub mod response;

use pyo3::prelude::*;
use crate::dynamic::{get_model_class_class, get_model_object_class, get_ctx_class, setup_dynamic_container};

use crate::app::app::App;
use crate::namespace::namespace::Namespace;


#[pymodule]
fn teo(_py: Python, m: &PyModule) -> PyResult<()> {
    setup_dynamic_container()?;
    #[pyfunction]
    fn fetch_model_class_class(name: &str, py: Python) -> PyResult<PyObject> {
        get_model_class_class(name, py)
    }
    #[pyfunction]
    fn fetch_model_object_class(name: &str, py: Python) -> PyResult<PyObject> {
        get_model_object_class(name, py)
    }
    #[pyfunction]
    fn fetch_ctx_class(name: &str, py: Python) -> PyResult<PyObject> {
        get_ctx_class(name, py)
    }
    m.add_function(wrap_pyfunction!(fetch_model_class_class, m)?)?;
    m.add_function(wrap_pyfunction!(fetch_model_object_class, m)?)?;
    m.add_function(wrap_pyfunction!(fetch_ctx_class, m)?)?;
    m.add_class::<App>()?;
    m.add_class::<Namespace>()?;
    Ok(())
}
