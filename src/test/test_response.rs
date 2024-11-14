use pyo3::{pyclass, pymethods};

#[pyclass]
pub struct TestResponse {
    teo_test_response: teo::server::test_response::TestResponse,
}

#[pymethods]
impl TestResponse {
    
}