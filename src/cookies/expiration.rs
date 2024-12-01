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
    pub fn create_session() -> Self {
        Expiration { original: OriginalExpiration::Session }
    }

    #[staticmethod]
    pub fn create_datetime(datetime: DateTime<Utc>) -> Self {
        let timestamp = datetime.timestamp_millis();
        Self {
            original: OriginalExpiration::DateTime(OffsetDateTime::from_unix_timestamp(timestamp).unwrap())
        }
    }

    pub fn is_session(&self) -> bool {
        self.original.is_session()
    }

    pub fn is_datetime(&self) -> bool {
        self.original.is_datetime()
    }

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