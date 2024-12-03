use bson::oid::ObjectId as OriginalObjectId;
use pyo3::{pyclass, pymethods, PyResult};
use teo_result::Error;

#[pyclass]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct ObjectId {
    pub(crate) original: OriginalObjectId,
}


#[pymethods]
impl ObjectId {

    #[new]
    pub fn new(value: &str) -> PyResult<Self> {
        Ok(ObjectId { original: OriginalObjectId::parse_str(value).map_err(|e| Error::from(e))? })
    }

    pub fn __str__(&self) -> String {
        self.original.to_hex()
    }

    pub fn __repr__(&self) -> String {
        format!("teo.ObjectId('{}')", self.original.to_hex())
    }
}