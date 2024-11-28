pub mod object_id;
pub mod file;
pub mod range;
pub mod option_variant;
pub mod decimal;

use indexmap::IndexMap;
use pyo3::{exceptions::PyValueError, types::{PyAnyMethods, PyDict, PyDictMethods, PyList, PyListMethods, PyNone, PyString, PyTuple, PyTupleMethods}, Bound, IntoPyObjectExt, PyAny, PyObject, PyResult, Python};
use regex::Regex;
use teo::prelude::Value;
pub use object_id::ObjectId;
pub use file::File;
pub use range::Range;
pub use option_variant::OptionVariant;
use std::str::FromStr;
use bigdecimal::BigDecimal;
use pyo3::types::{PyBool, PyDate, PyDateTime, PyFloat, PyInt};
use chrono::prelude::{NaiveDate, Utc, DateTime};
use crate::dynamic::py_class_lookup_map::PYClassLookupMap;

use self::decimal::big_decimal_to_python_decimal;
use super::{array::teo_array_to_py_any, interface_enum_variant::teo_interface_enum_variant_to_py_any, model::teo_model_object_to_py_any, pipeline::teo_pipeline_to_py_any, r#struct::teo_struct_object_to_py_any};

pub fn teo_value_to_py_any_without_model_objects<'p>(py: Python<'p>, value: &Value) -> PyResult<PyObject> {
    Ok(match value {
        Value::Null => PyNone::get(py).as_unbound().clone_ref(py).into_any(),
        Value::ObjectId(oid) => ObjectId { value: oid.clone() }.into_py_any(py)?,
        Value::String(s) => s.into_py_any(py)?,
        Value::Int(i) => i.into_py_any(py)?,
        Value::Int64(i) => i.into_py_any(py)?,
        Value::Float32(f) => f.into_py_any(py)?,
        Value::Float(f) => f.into_py_any(py)?,
        Value::Bool(b) => b.into_py_any(py)?,
        Value::Date(d) => d.into_py_any(py)?,
        Value::DateTime(d) => d.into_py_any(py)?,
        Value::Decimal(b) => big_decimal_to_python_decimal(b.clone(), py)?,
        Value::Array(v) => {
            let list = PyList::empty(py);
            for value in v {
                list.append(teo_value_to_py_any_without_model_objects(py, value)?)?;
            }
            list.into_py_any(py)?
        },

        Value::Dictionary(m) => {
            let dict = PyDict::new(py);
            for (k, v) in m {
                dict.set_item(k, teo_value_to_py_any_without_model_objects(py, v)?)?;
            }
            dict.into_py_any(py)?
        },
        Value::Range(range) => {
            let instance = Range { value: range.clone() };
            instance.into_py_any(py)?
        }
        Value::Tuple(tuple) => {
            PyTuple::new(py, tuple.iter().map(|v| teo_value_to_py_any_without_model_objects(py, v)).collect::<PyResult<Vec<PyObject>>>()?)?.into_py_any(py)?
        }
        Value::OptionVariant(option_variant) => {
            let instance = OptionVariant { value: option_variant.clone() };
            instance.into_py_any(py)?
        }
        Value::Regex(regex) => {
            let re = py.import("re")?;
            let compile = re.getattr("compile")?;
            let result = compile.call((regex.as_str(),), None)?;
            result.into_py_any(py)?
        }
        Value::File(file) => {
            let instance = File::from(file);
            instance.into_py_any(py)?
        }
        Value::StructObject(struct_object) => teo_struct_object_to_py_any(struct_object)?,
        Value::Pipeline(pipeline) => teo_pipeline_to_py_any(py, pipeline)?,
        Value::InterfaceEnumVariant(interface_enum_variant) => teo_interface_enum_variant_to_py_any(py, interface_enum_variant)?,
        _ => Err(PyValueError::new_err("cannot convert Teo value to Python value"))?,
    })
}

pub(crate) fn teo_value_to_py_any<'p>(py: Python<'p>, value: &Value, map: &PYClassLookupMap) -> PyResult<PyObject> {
    Ok(match value {
        Value::ModelObject(model_object) => teo_model_object_to_py_any(py, model_object, map)?,
        Value::Array(v) => teo_array_to_py_any(py, v, map)?,
        Value::Dictionary(m) => {
            let dict = PyDict::new(py);
            for (k, v) in m {
                dict.set_item(k, teo_value_to_py_any(py, v, map)?)?;
            }
            dict.into_py_any(py)?
        },
        Value::Tuple(tuple) => {
            PyTuple::new(py, tuple.iter().map(|v| teo_value_to_py_any(py, v, map)).collect::<PyResult<Vec<PyObject>>>()?)?.into_py_any(py)?
        },
        _ => teo_value_to_py_any_without_model_objects(py, value)?,
    })
}

pub fn py_any_to_teo_value(py: Python<'_>, object: &Bound<PyAny>) -> PyResult<Value> {
    if object.is_none() {
        Ok(Value::Null)
    } else if object.is_instance_of::<ObjectId>() {
        let object_id: ObjectId = object.extract()?;
        Ok(Value::ObjectId(object_id.value.clone()))
    } else if object.is_instance_of::<PyString>() {
        let s: String = object.extract()?;
        Ok(Value::String(s))
    } else if object.is_instance_of::<PyBool>() {
        let b: bool = object.extract()?;
        Ok(Value::Bool(b))
    } else if object.is_instance_of::<PyInt>() {
        let i: i64 = object.extract()?;
        Ok(Value::Int64(i))
    } else if object.is_instance_of::<PyFloat>() {
        let f: f64 = object.extract()?;
        Ok(Value::Float(f))
    } else if object.is_instance_of::<PyDateTime>() {
        let d: DateTime<Utc> = object.extract()?;
        Ok(Value::DateTime(d))
    } else if object.is_instance_of::<PyDate>() {
        let d: NaiveDate = object.extract()?;
        Ok(Value::Date(d))
    } else if object.is_instance_of::<PyList>() {
        let v: Vec<Bound<PyAny>> = object.extract()?;
        let mut vec = vec![];
        for value in v {
            vec.push(py_any_to_teo_value(py, &value)?);
        }
        Ok(Value::Array(vec))
    } else if object.is_instance_of::<PyDict>() {
        let dict: &Bound<PyDict> = object.downcast()?;
        let mut map: IndexMap<String, Value> = IndexMap::new();
        for k in dict.keys() {
            let k_str: &str = k.extract()?;
            map.insert(k_str.to_owned(), py_any_to_teo_value(py, &dict.get_item(k)?.unwrap())?);
        }
        Ok(Value::Dictionary(map))
    } else if object.is_instance_of::<Range>() {
        let range: Range = object.extract()?;
        Ok(Value::Range(range.value.clone()))
    } else if object.is_instance_of::<PyTuple>() {
        let tuple: &Bound<PyTuple> = object.downcast()?;
        let mut vec = vec![];
        for value in tuple.iter() {
            vec.push(py_any_to_teo_value(py, &value)?);
        }
        Ok(Value::Tuple(vec))

    } else if object.is_instance_of::<File>() {
        let file: File = object.extract()?;
        Ok(Value::File((&file).into()))
    } else {
        let decimal_module = py.import("decimal")?;
        let decimal_class = decimal_module.getattr("Decimal")?;
        let re_module = py.import("re")?;
        let pattern_class = re_module.getattr("Pattern")?;
        if object.is_instance(&decimal_class)? {
            let s: String = object.call_method0("__str__")?.extract()?;
            Ok(Value::Decimal(BigDecimal::from_str(&s).unwrap()))
        } else if object.is_instance(&pattern_class)? {
            let pattern_any = object.getattr("pattern")?.into_py_any(py)?;
            let pattern_str: &str = pattern_any.extract(py)?;
            let r: Regex = Regex::new(pattern_str).unwrap();
            Ok(Value::Regex(r))
        }
         else {
            Err(PyValueError::new_err("Cannot convert Python object to Teo value."))
        }
    }
}
