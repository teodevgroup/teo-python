use pyo3::{pyclass, pymethods};
use super::{TestRequest, TestResponse};

#[pyclass]
pub struct TestServer {
    server: teo::server::server::Server,
}

#[pymethods]
impl TestServer {

}