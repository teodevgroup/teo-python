use pyo3::{pyclass, pymethods, Bound, PyAny, PyResult, Python};
use pyo3_async_runtimes::tokio::future_into_py;
use tokio::runtime::Builder;
use crate::{app::app::App, dynamic::synthesize_dynamic_python_classes};
use super::{TestRequest, TestResponse};

#[pyclass]
pub struct TestServer {
    server: teo::server::server::Server,
}

#[pymethods]
impl TestServer {

    #[new]
    pub fn new(app: &App) -> Self {
        let mut builder = Builder::new_multi_thread();
        builder.enable_all();
        pyo3_async_runtimes::tokio::init(builder);
        Self { 
            server: teo::server::server::Server::new(app.teo_app.clone())
        }
    }

    pub fn setup<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        let static_self: &'static TestServer = unsafe { &*(self as * const TestServer) };
        let coroutine = future_into_py(py, (move || async {
            static_self.server.setup_app_for_unit_test().await?;
            Python::with_gil(|py| {
                synthesize_dynamic_python_classes(&static_self.server.app, py)
            })?;
            Ok(())
        })())?;
        Ok(coroutine)
    }

    pub fn reset<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        let static_self: &'static TestServer = unsafe { &*(self as * const TestServer) };
        let coroutine = future_into_py(py, (move || async {
            Ok(static_self.server.reset_app_for_unit_test().await?)
        })())?;
        Ok(coroutine)
    }

    pub fn process<'py>(&self, request: &TestRequest, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        let static_self: &'static TestServer = unsafe { &*(self as * const TestServer) };
        let static_request: &'static TestRequest = unsafe { &*(request as * const TestRequest) };
        let coroutine = future_into_py(py, (move || async {
            let hyper_request = static_request.to_hyper_request();
            let response = static_self.server.process_test_request_with_hyper_request(hyper_request).await?;
            Ok(TestResponse::new(response))
        })())?;
        Ok(coroutine)
    }
}