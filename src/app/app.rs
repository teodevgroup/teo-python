use pyo3::{pyclass, pymethods, types::{PyAnyMethods, PyList, PyListMethods, PyModule}, Bound, IntoPyObject, PyAny, PyErr, PyObject, PyResult, Python};
use pyo3_async_runtimes::tokio::future_into_py;
use ::teo::prelude::{RuntimeVersion, App as TeoApp, Entrance, transaction};
use teo_result::Error;
use tokio::runtime::Builder;

use crate::{dynamic::{synthesize_dynamic_python_classes, DynamicClasses, QueryDynamicClasses}, namespace::namespace::Namespace, utils::{check_callable::check_callable, is_coroutine::is_coroutine}};

#[pyclass]
#[derive(Clone)]
pub struct App {
    pub(crate) teo_app: TeoApp,
}

#[pymethods]
impl App {

    #[new]
    #[pyo3(signature = (argv=None))]
    fn new(py: Python<'_>, argv: Option<Vec<String>>) -> PyResult<Self> {
        Self::with_cli(py, false, argv)
    }

    #[staticmethod]
    #[pyo3(signature = (cli=true, argv=None))]
    fn with_cli(py: Python<'_>, cli: bool, argv: Option<Vec<String>>) -> PyResult<Self> {
        let platform = PyModule::import(py, "platform")?;
        let python_version = platform.getattr("python_version")?;
        let version_any = python_version.call0()?;
        let version_str: &str = version_any.extract::<&str>()?;
        let environment_version = RuntimeVersion::Python(version_str.to_owned());
        let entrance = if cli { Entrance::CLI } else { Entrance::APP };
        let rust_argv = match argv {
            Some(argv) => argv,
            None => {
                let sys = PyModule::import(py, "sys")?;
                let argv_binding = sys.getattr("argv")?;
                let argv: &Bound<PyList> = argv_binding.downcast()?;
                let mut rust_argv: Vec<String> = argv.iter().map(|s| s.to_string()).collect();
                rust_argv.insert(0, "python".to_string());
                rust_argv
            }
        };
        Ok(App { 
            teo_app: TeoApp::new_with_entrance_and_runtime_version(Some(entrance), Some(environment_version), Some(rust_argv)).unwrap() 
        })
    }

    fn setup<'p>(&mut self, py: Python<'p>, callback: PyObject) -> PyResult<()> {
        check_callable(&callback.bind(py))?;
        let app_data = self.teo_app.app_data().clone();
        self.teo_app.setup(move |ctx: transaction::Ctx| {
            let app_data = app_data.clone();
            let callback = Python::with_gil(|py| {
                callback.clone_ref(py)
            });
            async move {
                let dynamic_classes = DynamicClasses::retrieve(&app_data)?;
                let transformed = Python::with_gil(|py| {
                    let transformed_py = callback.call1(py, (dynamic_classes.teo_transaction_ctx_to_py_ctx_object(py, ctx, "")?,))?;
                    let is_coroutine = is_coroutine(&transformed_py)?;
                    Ok::<_, Error>((transformed_py, is_coroutine))
                })?;
                if transformed.1 {
                    let fut = Python::with_gil(|py| {
                        pyo3_async_runtimes::tokio::into_future(transformed.0.bind(py).clone())
                    })?;
                    let _ = fut.await?;
                }
                Ok(())    
            }
        });
        Ok(())
    }

    #[pyo3(signature = (name, desc, callback))]
    fn program<'p>(&mut self, py: Python<'p>, name: &str, desc: Option<&str>, callback: PyObject) -> PyResult<()> {
        check_callable(&callback.bind(py))?;
        let app_data = self.teo_app.app_data().clone();
        self.teo_app.program(name, desc, move |ctx: transaction::Ctx| {
            let app_data = app_data.clone();
            let callback = Python::with_gil(|py| {
                callback.clone_ref(py)
            });
            async move {
                let dynamic_classes = DynamicClasses::retrieve(&app_data)?;
                let transformed = Python::with_gil(|py| {
                    let transformed_py = callback.call1(py, (dynamic_classes.teo_transaction_ctx_to_py_ctx_object(py, ctx, "")?,))?;
                    let is_coroutine = is_coroutine(&transformed_py)?;
                    Ok::<_, Error>((transformed_py, is_coroutine))
                })?;
                if transformed.1 {
                    let fut = Python::with_gil(|py| {
                        pyo3_async_runtimes::tokio::into_future(transformed.0.bind(py).clone())
                    })?;
                    let _ = fut.await?;
                }
                Ok(())
            }
        });
        Ok(())
    }

    fn _run<'a>(&self, py: Python<'a>) -> PyResult<Bound<'a, PyAny>> {
        let mut builder = Builder::new_multi_thread();
        builder.enable_all();
        pyo3_async_runtimes::tokio::init(builder);
        let cloned_self = self.clone();
        let coroutine = future_into_py(py, (move || {
            let cloned_self = cloned_self.clone();
            async move {
                cloned_self.teo_app.prepare_for_run().await?;
                Python::with_gil(|py| {
                    synthesize_dynamic_python_classes(&cloned_self.teo_app, py)?;
                    Ok::<(), PyErr>(())
                })?;
                let mut builder = Builder::new_multi_thread();
                builder.enable_all();
                pyo3_async_runtimes::tokio::init(builder);
                cloned_self.teo_app.run_without_prepare().await?;
                Ok(())    
            }
        })())?;
        Ok::<Bound<PyAny>, PyErr>(coroutine.into_pyobject(py)?)
    }

    #[getter]
    fn main_namespace(&self) -> Namespace {
        Namespace { builder: self.teo_app.main_namespace().clone() }
    }

    #[getter]
    fn main(&self) -> Namespace {
        Namespace { builder: self.teo_app.main_namespace().clone() }
    }
}