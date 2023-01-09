use std::sync::Arc;
use pyo3::prelude::*;
use pyo3_asyncio::async_std::future_into_py;
use ::teo::core::app::builder::AppBuilder as TeoAppBuilder;
use ::teo::core::app::App as TeoApp;
use ::teo::core::app::environment::EnvironmentVersion;
use to_mut::ToMut;

#[pyclass]
struct AppBuilder {
    app_builder: Arc<TeoAppBuilder>,
}

#[pymethods]
impl AppBuilder {

    #[new]
    fn new(py: Python<'_>) -> Self {
        let platform = PyModule::import(py, "platform").unwrap();
        let python_version: Py<PyAny> = platform.getattr("python_version").unwrap().into();
        let version_any = python_version.call0(py).unwrap();
        let version_str: &str = version_any.extract::<&str>(py).unwrap();
        let environment_version = EnvironmentVersion::Python(version_str.to_owned());
        AppBuilder { app_builder: Arc::new(TeoAppBuilder::new_with_environment_version(environment_version)) }
    }

    fn load(&self, schema_file_name: Option<&str>) -> PyResult<()> {
        let app_builder_ref = self.app_builder.as_ref();
        let app_builder_mut_ref = app_builder_ref.to_mut();
        app_builder_mut_ref.load(schema_file_name);
        Ok(())
    }

    fn build<'p>(&self, py: Python<'p>) -> PyResult<&'p PyAny> {
        let app_builder = self.app_builder.clone();
        future_into_py(py, async move {
            let teo_app = Arc::new(app_builder.build().await);
            Ok(App { app: teo_app })
        })
    }
}

#[pyclass]
struct App {
    app: Arc<TeoApp>,
}

#[pymethods]
impl App {
    fn run<'p>(&self, py: Python<'p>) -> PyResult<&'p PyAny> {
        let app = self.app.clone();
        future_into_py(py, async move {
            app.run().await;
            Ok(Python::with_gil(|_py| {
                ()
            }))
        })
    }
}

#[pymodule]
fn teo(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<AppBuilder>()?;
    m.add_class::<App>()?;
    Ok(())
}
