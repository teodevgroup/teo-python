use teo::prelude::OptionVariant as OriginalOptionVariant;
use pyo3::{pyclass, pymethods, PyResult};

#[pyclass]
pub struct OptionVariant {
    pub(crate) original: OriginalOptionVariant
}

#[pymethods]
impl OptionVariant {

    pub fn __int__(&self) -> i32 {
        self.original.value
    }

    pub fn __bool__(&self) -> bool {
        self.original.value != 0
    }

    pub fn __str__(&self) -> String {
        self.original.display.clone()
    }

    pub fn __repr__(&self) -> String {
        format!("teo.OptionVariant(value: {}, display: `{}`)", self.original.value, self.original.display)
    }

    pub fn __and__(&self, rhs: &Self) -> PyResult<Self> {
        Ok(OptionVariant {
            original: (&self.original & &rhs.original)?
        })
    }

    pub fn __or__(&self, rhs: &Self) -> PyResult<Self> {
        Ok(OptionVariant {
            original: (&self.original | &rhs.original)?
        })
    }

    pub fn __xor__(&self, rhs: &Self) -> PyResult<Self> {
        Ok(OptionVariant {
            original: (&self.original ^ &rhs.original)?
        })
    }

    pub fn __invert__(&self) -> Self {
        OptionVariant {
            original: !&self.original
        }
    }
}