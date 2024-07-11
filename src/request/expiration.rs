use chrono::{DateTime, Utc};
use pyo3::{pyclass, pymethods};
use time::OffsetDateTime;
use teo::prelude::request::Expiration as ActixExpiration;

#[pyclass]
#[derive(Clone)]
pub struct Expiration {
    pub(crate) inner: ActixExpiration,
}

#[pymethods]
impl Expiration {

    #[staticmethod]
    pub fn create_session() -> Self {
        Self {
            inner: ActixExpiration::Session
        }
    }

    #[staticmethod]
    pub fn create_datetime(datetime: DateTime<Utc>) -> Self {
        let timestamp = datetime.timestamp_millis();
        Self {
            inner: ActixExpiration::DateTime(OffsetDateTime::from_unix_timestamp(timestamp).unwrap())
        }
    }

    pub fn is_session(&self) -> bool {
        self.inner.is_session()
    }

    pub fn is_datetime(&self) -> bool {
        self.inner.is_datetime()
    }

    pub fn datetime(&self) -> Option<DateTime<Utc>> {
        match &self.inner {
            ActixExpiration::DateTime(offset) => {
                let timestamp = offset.unix_timestamp();
                let datetime = DateTime::from_timestamp_millis(timestamp);
                datetime
            },
            _ => None
        }
    }
}