use time::{self, OffsetDateTime};
use chrono::{DateTime, Utc};
use pyo3::prelude::*;
use teo::prelude::request::Expiration as TeoExpiration;

#[pyclass]
#[derive(Clone)]
pub struct Expiration {
    pub(crate) inner: TeoExpiration
}

#[pymethods]
impl Expiration {

    #[staticmethod]
    pub fn create_session() -> Self {
        Expiration { inner: TeoExpiration::Session }
    }

    #[staticmethod]
    pub fn create_datetime(datetime: DateTime<Utc>) -> Self {
        let timestamp = datetime.timestamp_millis();
        Self {
            inner: TeoExpiration::DateTime(OffsetDateTime::from_unix_timestamp(timestamp).unwrap())
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
            TeoExpiration::DateTime(offset) => {
                let timestamp = offset.unix_timestamp();
                let datetime = DateTime::from_timestamp_millis(timestamp);
                datetime
            },
            _ => None
        }
    }
}