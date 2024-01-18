use teo::prelude::EnumVariant as TeoEnumVariant;
use pyo3::{pyclass, pymethods};

#[pyclass]
pub struct EnumVariant {
    pub(crate) value: TeoEnumVariant
}

#[pymethods]
impl EnumVariant {

    pub fn to_string(&self) -> String {
        self.value.value.clone()
    }
    
    pub fn from_string(value: String) -> EnumVariant {
        Self { 
            value: TeoEnumVariant {
                value,
                args: None,
            } 
        }
    }
}

