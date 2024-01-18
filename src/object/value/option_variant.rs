use teo::prelude::OptionVariant as TeoOptionVariant;
use pyo3::pyclass;

#[pyclass]
pub struct OptionVariant {
    pub(crate) value: TeoOptionVariant
}
