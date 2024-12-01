use time::{self, OffsetDateTime};
use chrono::{DateTime, Utc};
use pyo3::prelude::*;
use teo::prelude::cookies::Expiration as OriginalExpiration;

#[pyclass]
#[derive(Clone)]
pub struct Expiration {
    pub(crate) original: OriginalExpiration
}

#[pymethods]
impl Expiration {

    #[staticmethod]
    pub fn session_expiration() -> Self {
        Expiration { original: OriginalExpiration::Session }
    }

    #[staticmethod]
    pub fn datetime_expiration(datetime: DateTime<Utc>) -> Self {
        let timestamp = datetime.timestamp_millis();
        Self {
            original: OriginalExpiration::DateTime(OffsetDateTime::from_unix_timestamp(timestamp).unwrap())
        }
    }

    #[getter]
    pub fn is_session(&self) -> bool {
        self.original.is_session()
    }

    #[getter]
    pub fn is_datetime(&self) -> bool {
        self.original.is_datetime()
    }

    #[getter]
    pub fn datetime(&self) -> Option<DateTime<Utc>> {
        match &self.original {
            OriginalExpiration::DateTime(offset) => {
                let timestamp = offset.unix_timestamp();
                let datetime = DateTime::from_timestamp_millis(timestamp);
                datetime
            },
            _ => None
        }
    }
}