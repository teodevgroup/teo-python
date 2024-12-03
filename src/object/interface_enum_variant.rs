use pyo3::{pyclass, pymethods, IntoPyObjectExt, PyObject, PyResult, Python};
use teo::prelude::{InterfaceEnumVariant as OriginalInterfaceEnumVariant, Value};

use super::value::teo_value_to_py_any_without_model_objects;

#[pyclass]
pub struct InterfaceEnumVariant {
    pub(crate) original: OriginalInterfaceEnumVariant
}

#[pymethods]
impl InterfaceEnumVariant {

    pub fn __str__(&self) -> String {
        format!("{}", self.original)
    }

    pub fn __repr__(&self) -> String {
        format!("{}", self.original)
    }

    #[getter]
    pub fn value(&self) -> &str {
        self.original.value()
    }

    pub fn __getitem__(&self, name: &str, py: Python<'_>) -> PyResult<Option<PyObject>> {
        if let Some(args) = self.original.args() {
            let value: Option<Value> = args.get_optional(name)?;
            if let Some(value) = value {
                Ok(Some(teo_value_to_py_any_without_model_objects(py, &value)?))
            } else {
                Ok(None)
            }
        }  else {
            Ok(None)
        }
    }
}

pub fn teo_interface_enum_variant_to_py_any(py: Python<'_>, interface_enum_variant: &OriginalInterfaceEnumVariant) -> PyResult<PyObject> {
    let instance = InterfaceEnumVariant { original: interface_enum_variant.clone() };
    Ok(instance.into_py_any(py)?)
}