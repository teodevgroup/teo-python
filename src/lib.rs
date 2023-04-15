pub mod convert;
pub mod utils;
pub mod result;
pub mod classes;

use std::sync::Arc;
use pyo3::prelude::*;
use pyo3_asyncio::tokio::future_into_py;
use ::teo::core::app::builder::AppBuilder as TeoAppBuilder;
use ::teo::core::app::environment::EnvironmentVersion;
use ::teo::prelude::Value;
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

    fn setup<'p>(&self, _py: Python<'p>, callback: &PyAny) -> PyResult<()> {
        let mut_builder = self.app_builder.as_ref().to_mut();
        check_callable(callback)?;
        let callback_owned = Box::leak(Box::new(Py::from(callback)));
        mut_builder.before_server_start(|| async {
            let transformed = Python::with_gil(|py| {
                let callback = callback_owned.as_ref(py);
                let transformed_py = callback.call0()?;
                let is_coroutine = is_coroutine(transformed_py, py)?;
                Ok((transformed_py.into_py(py), is_coroutine))
            }).into_teo_result()?;
            if transformed.1 {
                let fut = Python::with_gil(|py| {
                    pyo3_asyncio::tokio::into_future(transformed.0.as_ref(py))
                }).into_teo_result()?;
                let _ = fut.await.into_teo_result()?;
            }
            Ok(())
        });
        Ok(())
    }

    fn run<'p>(&self, py: Python<'p>) -> PyResult<&'p PyAny> {
        let app_builder = self.app_builder.clone();
        future_into_py(py, async move {
            let app_builder_ref = app_builder.as_ref().clone();
            let teo_app = app_builder_ref.build().await;
            Python::with_gil(|py| {
                generate_classes(&teo_app, py)
            })?;
            match teo_app.run().await {
                Ok(()) => Ok(()),
                Err(err) => Err(PyRuntimeError::new_err(err.to_string())),
            }
        })
    }

    fn transform(&self, _py: Python, name: &str, callback: &PyAny) -> PyResult<()> {
        check_callable(callback)?;
        let mut_builder = self.app_builder.as_ref().to_mut();
        let callback_owned = Box::leak(Box::new(Py::from(callback)));
        mut_builder.transform(name, |value: Value| async {
            Python::with_gil(|py| {
                let callback = callback_owned.as_ref(py);
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
        check_callable(callback)?;
        let mut_builder = self.app_builder.as_ref().to_mut();
        let callback_owned = Box::leak(Box::new(Py::from(callback)));
        mut_builder.validate(name, |value: Value| async {
            Python::with_gil(|py| {
                let callback = callback_owned.as_ref(py);
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

    fn callback(&self, _py: Python, name: &str, callback: &PyAny) -> PyResult<()> {
        check_callable(callback)?;
        let mut_builder = self.app_builder.as_ref().to_mut();
        let callback_owned = Box::leak(Box::new(Py::from(callback)));
        mut_builder.callback(name, |value: Value| async {
            Python::with_gil(|py| {
                let callback = callback_owned.as_ref(py);
                let py_object = teo_value_to_py_object(value, py)?;
                let transformed_py = callback.call1((py_object,))?;
                let _ = await_coroutine_if_needed(transformed_py, py)?;
                Ok(())
            }).into_teo_result()
        });
        Ok(())
    }

    fn compare(&self, _py: Python, name: &str, callback: &PyAny) -> PyResult<()> {
        check_callable(callback)?;
        let mut_builder = self.app_builder.as_ref().to_mut();
        let callback_owned = Box::leak(Box::new(Py::from(callback)));
        mut_builder.compare(name, |old: Value, new: Value| async {
            Python::with_gil(|py| {
                let callback = callback_owned.as_ref(py);
                let py_object_old = teo_value_to_py_object(old, py)?;
                let py_object_new = teo_value_to_py_object(new, py)?;
                let transformed_py = callback.call1((py_object_old,py_object_new))?;
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
    setup_classes_container()?;
    #[pyfunction]
    fn fetch_model_class(name: &str, py: Python) -> PyResult<PyObject> {
        get_model_class(name, py)
    }
    m.add_function(wrap_pyfunction!(fetch_model_class, m)?)?;
    m.add_class::<App>()?;
    Ok(())
}
