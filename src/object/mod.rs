pub mod value;
pub mod pipeline;
pub mod r#struct;
pub mod arguments;
pub mod model;
pub mod interface_enum_variant;
pub mod array;

use pyo3::{Python, PyResult, PyObject};
use teo::prelude::object::{Object as TeoObject, ObjectInner};
use self::array::teo_array_to_py_any;
use self::interface_enum_variant::teo_interface_enum_variant_to_py_any;
use self::model::teo_model_object_to_py_any;
use self::pipeline::teo_pipeline_to_py_any;
use self::r#struct::teo_struct_object_to_py_any;
use self::value::{teo_value_to_py_any, py_any_to_teo_value};

pub fn teo_object_to_py_any(py: Python<'_>, object: &TeoObject) -> PyResult<PyObject> {
    match object.inner.as_ref() {
        ObjectInner::Teon(value) => teo_value_to_py_any(py, value),
        ObjectInner::ModelObject(model_object) => teo_model_object_to_py_any(py, model_object),
        ObjectInner::StructObject(struct_object) => teo_struct_object_to_py_any(struct_object),
        ObjectInner::Pipeline(pipeline) => teo_pipeline_to_py_any(py, pipeline),
        ObjectInner::InterfaceEnumVariant(interface_enum_variant) => teo_interface_enum_variant_to_py_any(py, interface_enum_variant),
        ObjectInner::Array(array) => teo_array_to_py_any(py, array),
    }
}

pub fn py_any_to_teo_object(py: Python<'_>, object: PyObject) -> PyResult<TeoObject> {
    Ok(TeoObject::from(py_any_to_teo_value(py, object.as_ref(py))?))
}
