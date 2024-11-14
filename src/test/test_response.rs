use pyo3::{pyclass, pymethods, types::{PyAnyMethods, PyBytes}, Bound, PyAny, PyResult, Python};

#[pyclass]
pub struct TestResponse {
    teo_test_response: teo::server::test_response::TestResponse,
}

#[pymethods]
impl TestResponse {
    
    pub fn status(&self) -> u16 {
        self.teo_test_response.status().as_u16()
    }

    pub fn version(&self) -> String {
        format!("{:?}", self.teo_test_response.version())
    }

    pub fn body<'py>(&self, py: Python<'py>) -> Bound<'py, PyBytes> {
        let data = Vec::<u8>::from(self.teo_test_response.body().clone());
        PyBytes::new_bound(py, &data)
    }

    pub fn body_as_string(&self) -> String {
        self.teo_test_response.body_as_string()
    }

    pub fn body_as_json<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        let string = self.teo_test_response.body_as_string();
        let json = py.import_bound("json")?;
        let loads = json.getattr("loads")?;
        let result = loads.call1((string,))?;
        Ok(result)
    }

    pub fn contains_header(&self, name: &str) -> bool {
        self.teo_test_response.headers().contains_key(name)
    }

    pub fn header_value(&self, name: &str) -> PyResult<Option<&str>> {
        let header_value = self.teo_test_response.headers().get(name);
        match header_value {
            None => Ok(None),
            Some(header_value) => {
                let header_value = header_value.to_str().map_err(|_| {
                    teo_result::Error::internal_server_error_message(format!("cannot read request header value: {}", name))
                })?;
                Ok(Some(header_value))
            }
        }
    }

    pub fn header_values(&self, name: &str) -> PyResult<Vec<&str>> {
        let header_values = self.teo_test_response.headers().get_all(name);
        let mut result = Vec::new();
        for header_value in header_values {
            let header_value = header_value.to_str().map_err(|_| {
                teo_result::Error::internal_server_error_message(format!("cannot read request header value: {}", name))
            })?;
            result.push(header_value);
        }
        Ok(result)
    }

    pub fn header_keys(&self) -> Vec<&str> {
        let header_map = self.teo_test_response.headers();
        let mut result = vec![];
        header_map.keys().for_each(|k| {
            result.push(k.as_str());
        });
        result
    }

    pub fn headers_length(&self) -> i64 {
        self.teo_test_response.headers().len() as i64
    }
}

impl TestResponse {
    pub(super) fn new(teo_test_response: teo::server::test_response::TestResponse) -> Self {
        Self {
            teo_test_response,
        }
    }
}