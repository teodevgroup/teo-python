use pyo3::{pyclass, IntoPyObjectExt, PyObject, PyResult, Python};
use teo::prelude::InterfaceEnumVariant as TeoInterfaceEnumVariant;

#[pyclass]
pub struct InterfaceEnumVariant {
    pub(crate) value: TeoInterfaceEnumVariant
}

pub fn teo_interface_enum_variant_to_py_any(py: Python<'_>, interface_enum_variant: &TeoInterfaceEnumVariant) -> PyResult<PyObject> {
    let instance = InterfaceEnumVariant { value: interface_enum_variant.clone() };
    Ok(instance.into_py_any(py)?)
}