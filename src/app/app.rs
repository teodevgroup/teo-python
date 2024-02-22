use pyo3::{pyclass, pymethods, types::{PyList, PyModule}, IntoPy, Py, PyAny, PyErr, PyObject, PyResult, Python};
use pyo3_asyncio::tokio::future_into_py;
use teo::cli::runtime_version::RuntimeVersion;
use ::teo::prelude::{App as TeoApp, Entrance, transaction};
use tokio::runtime::Builder;

use crate::{utils::{check_callable::check_callable, is_coroutine::is_coroutine}, result::{IntoTeoResult, IntoPyResultWithGil}, namespace::namespace::Namespace, dynamic::{synthesize_dynamic_python_classes, py_ctx_object_from_teo_transaction_ctx}};

#[pyclass]
pub struct App {
    teo_app: TeoApp,
}

#[pymethods]
impl App {

    #[new]
    fn new(py: Python<'_>) -> PyResult<Self> {
        Self::with_cli(py, false)
    }

    #[staticmethod]
    fn with_cli(py: Python<'_>, cli: bool) -> PyResult<Self> {
        let platform = PyModule::import(py, "platform")?;
        let python_version: Py<PyAny> = platform.getattr("python_version")?.into();
        let version_any = python_version.call0(py)?;
        let version_str: &str = version_any.extract::<&str>(py)?;
        let environment_version = RuntimeVersion::Python(version_str.to_owned());
        let entrance = if cli { Entrance::CLI } else { Entrance::APP };
        let sys = PyModule::import(py, "sys")?;
        let argv: &PyList = sys.getattr("argv")?.extract()?;
        let mut rust_argv: Vec<String> = argv.iter().map(|s| s.to_string()).collect();
        rust_argv.insert(0, "python".to_string());
        Ok(App { 
            teo_app: TeoApp::new_with_entrance_and_runtime_version(Some(entrance), Some(environment_version), Some(rust_argv)).unwrap() 
        })
    }

    fn setup<'p>(&self, _py: Python<'p>, callback: &PyAny) -> PyResult<()> {
        check_callable(callback)?;
        let callback_owned = Box::leak(Box::new(Py::from(callback)));
        self.teo_app.setup(|ctx: transaction::Ctx| async {
            let transformed = Python::with_gil(|py| {
                let callback = callback_owned.as_ref(py);
                let transformed_py = callback.call1((py_ctx_object_from_teo_transaction_ctx(py, ctx, "")?,))?.into_py(py);
                let is_coroutine = is_coroutine(transformed_py.as_ref(py))?;
                Ok((transformed_py, is_coroutine))
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
                let transformed_py = callback.call1((py_ctx_object_from_teo_transaction_ctx(py, ctx, "")?,))?.into_py(py);
                let is_coroutine = is_coroutine(transformed_py.as_ref(py))?;
                Ok((transformed_py, is_coroutine))
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

    fn _run(&self, py: Python<'_>) -> PyResult<PyObject> {
        let mut builder = Builder::new_multi_thread();
        builder.enable_all();
        pyo3_asyncio::tokio::init(builder);
        let static_self: &'static App = unsafe { &*(self as * const App) };
        let coroutine = future_into_py(py, (|| async move {
            static_self.teo_app.prepare_for_run().await.into_py_result_with_gil()?;
            Python::with_gil(|py| {
                synthesize_dynamic_python_classes(py, &static_self.teo_app)?;
                Ok::<(), PyErr>(())
            })?;
            static_self.teo_app.run_without_prepare().await.into_py_result_with_gil()?;
            Ok(())
        })())?;
        Ok::<PyObject, PyErr>(coroutine.into_py(py))
    }

    fn main_namespace(&self) -> Namespace {
        Namespace { teo_namespace: self.teo_app.main_namespace_mut() }
    }
}