use pyo3::{pyclass, pymethods, Python, PyResult, types::{PyModule, PyType}, PyAny, Py, exceptions::PyRuntimeError, PyErr};
use pyo3_asyncio::generic::future_into_py;
use teo::cli::runtime_version::RuntimeVersion;
use ::teo::prelude::{App as TeoApp, Entrance, transaction};

use crate::{utils::{check_callable::check_callable, is_coroutine::is_coroutine}, result::{IntoPyResult, IntoTeoResult, IntoPyResultWithGil}, namespace::namespace::Namespace, dynamic::synthesize_dynamic_python_classes};

#[pyclass]
pub struct App {
    teo_app: TeoApp,
}


#[pymethods]
impl App {

    #[new]
    fn new(cls: &PyType, py: Python<'_>) -> PyResult<Self> {
        Self::with_cli(cls, py, false)
    }

    #[classmethod]
    fn with_cli(cls: &PyType, py: Python<'_>, cli: bool) -> PyResult<Self> {
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
                Ok((transformed_py, is_coroutine))
            }).into_teo_result()?;
            if transformed.1 {
                let fut = Python::with_gil(|py| {
                    pyo3_asyncio::tokio::into_future(transformed.0)
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
                Ok((transformed_py, is_coroutine))
            }).into_teo_result()?;
            if transformed.1 {
                let fut = Python::with_gil(|py| {
                    pyo3_asyncio::tokio::into_future(transformed.0)
                }).into_teo_result()?;
                let _ = fut.await.into_teo_result()?;
            }
            Ok(())
        });
        Ok(())
    }

    fn run<'p>(&'static self, py: Python<'p>) -> PyResult<&'p PyAny> {
        future_into_py(py, async move {
            self.teo_app.prepare_for_run().await.into_py_result_with_gil()?;
            Python::with_gil(|py| {
                synthesize_dynamic_python_classes(py, &self.teo_app)?;
                Ok(())
            })?;
            match self.teo_app.run_without_prepare().await {
                Ok(()) => Ok::<(), PyErr>(()),
                Err(err) => Err(PyRuntimeError::new_err(err.to_string())),
            }
        })
    }

    fn main_namespace(&'static self) -> Namespace {
        Namespace { teo_namespace: self.teo_app.main_namespace_mut() }
    }
}