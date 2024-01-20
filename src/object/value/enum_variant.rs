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
    
    #[classmethod]
    pub fn from_string(cls: &PyType, value: &str) -> EnumVariant {
        Self {
            value: TeoEnumVariant {
                value: value.to_owned(),
                args: None,
            } 
        }
    }
}

