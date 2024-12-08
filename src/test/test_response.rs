use pyo3::{pyclass, pymethods, types::{PyAnyMethods, PyBytes}, Bound, PyAny, PyResult, Python};
use teo::server::test_response::TestResponse as OriginalTestResponse;
use crate::{cookies::Cookies, headers::Headers};

#[pyclass]
pub struct TestResponse {
    original: OriginalTestResponse,
}

impl From<OriginalTestResponse> for TestResponse {
    fn from(original: OriginalTestResponse) -> Self {
        Self { original }
    }
}

#[pymethods]
impl TestResponse {

    #[getter]
    pub fn status(&self) -> u16 {
        self.original.status().as_u16()
    }

    #[getter]
    pub fn version(&self) -> String {
        format!("{:?}", self.original.version())
    }

    #[getter]
    pub fn headers(&self) -> Headers {
        Headers::from(self.original.headers().clone())
    }

    #[getter]
    pub fn cookies(&self) -> Cookies {
        Cookies::from(self.original.cookies().clone())
    }

    pub fn body<'py>(&self, py: Python<'py>) -> Bound<'py, PyBytes> {
        let data = Vec::<u8>::from(self.original.body().clone());
        PyBytes::new(py, &data)
    }

    pub fn body_as_string(&self) -> String {
        self.original.body_as_string()
    }

    pub fn body_as_json<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        let string = self.original.body_as_string();
        let json = py.import("json")?;
        let loads = json.getattr("loads")?;
        let result = loads.call1((string,))?;
        Ok(result)
    }
}
