use pyo3::{pyclass, pymethods, Python, PyResult, types::PyModule, PyAny, Py, exceptions::PyRuntimeError};
use pyo3_asyncio::generic::future_into_py;
use teo::cli::runtime_version::RuntimeVersion;
use ::teo::prelude::{Value, App as TeoApp, Entrance, transaction};

use crate::utils::{check_callable::check_callable, is_coroutine::is_coroutine};

#[pyclass]
struct App {
    teo_app: TeoApp,
}


#[pymethods]
impl App {

    #[new]
    fn new(py: Python<'_>) -> PyResult<Self> {
        Self::with_cli(py, false)
    }

    fn with_cli(py: Python<'_>, cli: bool) -> PyResult<Self> {
        let platform = PyModule::import(py, "platform")?;
        let python_version: Py<PyAny> = platform.getattr("python_version")?.into();
        let version_any = python_version.call0(py)?;
        let version_str: &str = version_any.extract::<&str>(py)?;
        let environment_version = RuntimeVersion::Python(version_str.to_owned());
        let entrance = if cli { Entrance::CLI } else { Entrance::APP };
        Ok(App { 
            teo_app: TeoApp::new_with_entrance_and_runtime_version(Some(entrance), Some(environment_version)).unwrap() 
        })
    }

    fn setup<'p>(&self, _py: Python<'p>, callback: &PyAny) -> PyResult<()> {
        check_callable(callback)?;
        let callback_owned = Box::leak(Box::new(Py::from(callback)));
        self.teo_app.setup(|ctx: transaction::Ctx| async {
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

    fn program<'p>(&self, _py: Python<'p>, name: &str, callback: &PyAny) -> PyResult<()> {
        check_callable(callback)?;
        let callback_owned = Box::leak(Box::new(Py::from(callback)));
        self.teo_app.program(name, |ctx: transaction::Ctx| async {
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