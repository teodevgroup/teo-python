use std::sync::Arc;
use pyo3::prelude::*;
use pyo3_asyncio::tokio::future_into_py;
use ::teo::core::app::builder::AppBuilder as TeoAppBuilder;
use ::teo::core::app::environment::EnvironmentVersion;
use to_mut::ToMut;

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
            let app_builder_ref = app_builder.as_ref();
            let app_builder_ref_mut = app_builder_ref.to_mut();
            let teo_app = app_builder_ref_mut.build().await;
            teo_app.run().await;
            Ok(Python::with_gil(|_py| {
                ()
            }))
        })
    }
}

#[pymodule]
fn teo(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<App>()?;
    Ok(())
}
