use pyo3::{pyclass, pymethods, types::{PyAnyMethods, PyList, PyModule}, IntoPy, Py, PyAny, PyErr, PyObject, PyResult, Python};
use pyo3_asyncio_0_21::tokio::future_into_py;
use ::teo::prelude::{RuntimeVersion, App as TeoApp, Entrance, transaction};
use teo_result::Error;
use tokio::runtime::Builder;

use crate::{dynamic::{py_class_lookup_map::PYClassLookupMap, synthesize_dynamic_python_classes}, namespace::namespace::Namespace, utils::{check_callable::check_callable, is_coroutine::is_coroutine}};

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
        let platform = PyModule::import_bound(py, "platform")?;
        let python_version: Py<PyAny> = platform.getattr("python_version")?.into();
        let version_any = python_version.call0(py)?;
        let version_str: &str = version_any.extract::<&str>(py)?;
        let environment_version = RuntimeVersion::Python(version_str.to_owned());
        let entrance = if cli { Entrance::CLI } else { Entrance::APP };
        let sys = PyModule::import_bound(py, "sys")?;
        let argv: &PyList = sys.getattr("argv")?.extract()?;
        let mut rust_argv: Vec<String> = argv.iter().map(|s| s.to_string()).collect();
        rust_argv.insert(0, "python".to_string());
        Ok(App { 
            teo_app: TeoApp::new_with_entrance_and_runtime_version(Some(entrance), Some(environment_version), Some(rust_argv)).unwrap() 
        })
    }

    fn setup<'p>(&mut self, py: Python<'p>, callback: PyObject) -> PyResult<()> {
        check_callable(&callback.bind(py))?;
        let callback = &*Box::leak(Box::new(callback));
        let map = PYClassLookupMap::from_app_data(self.teo_app.app_data()); 
        self.teo_app.setup(|ctx: transaction::Ctx| async {
            let transformed = Python::with_gil(|py| {
                let transformed_py = callback.call1(py, (map.teo_transaction_ctx_to_py_ctx_object(py, ctx, "")?,))?.into_py(py);
                let is_coroutine = is_coroutine(transformed_py.extract::<&PyAny>(py)?)?;
                Ok::<_, Error>((transformed_py, is_coroutine))
            })?;
            if transformed.1 {
                let fut = Python::with_gil(|py| {
                    pyo3_asyncio_0_21::tokio::into_future(transformed.0.bind(py).clone())
                })?;
                let _ = fut.await?;
            }
            Ok(())
        });
        Ok(())
    }

    #[pyo3(signature = (name, desc, callback))]
    fn program<'p>(&mut self, py: Python<'p>, name: &str, desc: Option<&str>, callback: PyObject) -> PyResult<()> {
        check_callable(&callback.bind(py))?;
        let callback_owned = &*Box::leak(Box::new(callback));
        let map = PYClassLookupMap::from_app_data(self.teo_app.app_data()); 
        self.teo_app.program(name, desc, |ctx: transaction::Ctx| async {
            let transformed = Python::with_gil(|py| {
                let transformed_py = callback_owned.call1(py, (map.teo_transaction_ctx_to_py_ctx_object(py, ctx, "")?,))?.into_py(py);
                let is_coroutine = is_coroutine(transformed_py.extract::<&PyAny>(py)?)?;
                Ok::<_, Error>((transformed_py, is_coroutine))
            })?;
            if transformed.1 {
                let fut = Python::with_gil(|py| {
                    pyo3_asyncio_0_21::tokio::into_future(transformed.0.bind(py).clone())
                })?;
                let _ = fut.await?;
            }
            Ok(())
        });
        Ok(())
    }

    fn _run(&self, py: Python<'_>) -> PyResult<PyObject> {
        let mut builder = Builder::new_multi_thread();
        builder.enable_all();
        pyo3_asyncio_0_21::tokio::init(builder);
        let static_self: &'static App = unsafe { &*(self as * const App) };
        let coroutine = future_into_py(py, (|| async move {
            static_self.teo_app.prepare_for_run().await?;
            Python::with_gil(|py| {
                synthesize_dynamic_python_classes(&static_self.teo_app, py)?;
                Ok::<(), PyErr>(())
            })?;
            static_self.teo_app.run_without_prepare().await?;
            Ok(())
        })())?;
        Ok::<PyObject, PyErr>(coroutine.into_py(py))
    }

    fn main_namespace(&self) -> Namespace {
        Namespace { teo_namespace: self.teo_app.main_namespace().clone() }
    }
}