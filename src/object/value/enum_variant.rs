use teo::prelude::EnumVariant as TeoEnumVariant;
use pyo3::{pyclass, pymethods, types::PyType};

#[pyclass]
pub struct EnumVariant {
    pub(crate) value: TeoEnumVariant
}

#[pymethods]
impl EnumVariant {

    pub fn to_string(&self) -> String {
        self.value.value.clone()
    }
    
    #[staticmethod]
    pub fn from_string(value: &str) -> EnumVariant {
        Self {
            value: TeoEnumVariant {
                value: value.to_owned(),
                args: None,
            } 
        }
    }
}

