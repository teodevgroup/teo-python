use bson::oid::ObjectId as BsonObjectId;
use pyo3::{pyclass, pymethods, PyResult, exceptions::PyValueError, types::PyType};

#[pyclass]
#[derive(Clone)]
pub struct ObjectId {
    pub(crate) value: BsonObjectId,
}


#[pymethods]
impl ObjectId {

    pub fn to_string(&self) -> String {
        self.value.to_hex()
    }

    #[staticmethod]
    pub fn from_string(string: &str) -> PyResult<ObjectId> {
        match BsonObjectId::parse_str(&string) {
            Ok(value) => Ok(Self { value }),
            Err(_) => Err(PyValueError::new_err("string doesn't represent valid ObjectId"))
        }
    }
}