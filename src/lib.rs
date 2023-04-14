pub mod convert;
pub mod utils;
pub mod result;

use std::sync::Arc;
use pyo3::prelude::*;
use pyo3_asyncio::tokio::future_into_py;
use ::teo::core::app::builder::AppBuilder as TeoAppBuilder;
use ::teo::core::app::environment::EnvironmentVersion;
use ::teo::prelude::Value;
use pyo3::exceptions::PyValueError;
use pyo3::ffi::{PyAsyncGen_Type, PyCoro_Type};
use pyo3::types::PyInt;
use to_mut::ToMut;
use crate::convert::to_py::teo_value_to_py_object;
use crate::convert::to_teo::py_object_to_teo_value;
use crate::utils::is_coroutine::is_coroutine;
use crate::result::IntoTeoResult;
use crate::utils::await_coroutine_if_needed::await_coroutine_if_needed;
use crate::utils::check_callable::check_callable;
use crate::utils::validate_result::validate_result;

#[pyclass]
struct App {
    app_builder: Arc<TeoAppBuilder>,
}

#[pymethods]
impl App {

    #[new]
    fn new(py: Python<'_>, cli: Option<bool>) -> Self {
        let platform = PyModule::import(py, "platform").unwrap();
        let python_version: Py<PyAny> = platform.getattr("python_version").unwrap().into();
        let version_any = python_version.call0(py).unwrap();
        let version_str: &str = version_any.extract::<&str>(py).unwrap();
        let environment_version = EnvironmentVersion::Python(version_str.to_owned());
        match cli {
            Some(cli) => if cli {
                App { app_builder: Arc::new(TeoAppBuilder::new_with_environment_version_and_entrance(environment_version, ::teo::core::app::entrance::Entrance::CLI)) }
            } else {
                App { app_builder: Arc::new(TeoAppBuilder::new_with_environment_version(environment_version)) }
            }
            None => App { app_builder: Arc::new(TeoAppBuilder::new_with_environment_version(environment_version)) }
        }
    }

    fn run<'p>(&self, py: Python<'p>) -> PyResult<&'p PyAny> {
        let app_builder = self.app_builder.clone();
        future_into_py(py, async move {
            let app_builder_ref = app_builder.as_ref().clone();
            let teo_app = app_builder_ref.build().await;
            teo_app.run().await;
            Ok(Python::with_gil(|_py| {
                ()
            }))
        })
    }

    fn transform(&self, _py: Python, name: &str, callback: &PyAny) -> PyResult<()> {
        let mut_builder = self.app_builder.as_ref().to_mut();
        let callback_owned = Box::leak(Box::new(Py::from(callback)));
        mut_builder.transform(name, |value: Value| async {
            Python::with_gil(|py| {
                let callback = callback_owned.as_ref(py);
                check_callable(callback)?;
                let py_object = teo_value_to_py_object(value, py)?;
                let transformed_py = callback.call1((py_object,))?;
                let transformed_py_awaited = await_coroutine_if_needed(transformed_py, py)?;
                let transformed_py_awaited_ref = transformed_py_awaited.as_ref(py);
                let transformed = py_object_to_teo_value(transformed_py_awaited_ref, py)?;
                Ok(transformed)
            }).into_teo_result()
        });
        Ok(())
    }

    fn validate(&self, _py: Python, name: &str, callback: &PyAny) -> PyResult<()> {
        let mut_builder = self.app_builder.as_ref().to_mut();
        let callback_owned = Box::leak(Box::new(Py::from(callback)));
        mut_builder.validate(name, |value: Value| async {
            Python::with_gil(|py| {
                let callback = callback_owned.as_ref(py);
                check_callable(callback)?;
                let py_object = teo_value_to_py_object(value, py)?;
                let transformed_py = callback.call1((py_object,))?;
                let transformed_py_awaited = await_coroutine_if_needed(transformed_py, py)?;
                let transformed_py_awaited_ref = transformed_py_awaited.as_ref(py);
                let transformed = py_object_to_teo_value(transformed_py_awaited_ref, py)?;
                Ok(validate_result(transformed))
            }).into_teo_result()?
        });
        Ok(())
    }
}

#[pymodule]
fn teo(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<App>()?;
    Ok(())
}
