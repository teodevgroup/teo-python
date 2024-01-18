pub mod object_id;
pub mod file;
pub mod range;
pub mod enum_variant;
pub mod option_variant;
pub mod decimal;

use indexmap::IndexMap;
use pyo3::{Python, PyAny, PyResult, IntoPy, types::{PyList, PyDict, PyTuple, PyString}, exceptions::PyValueError};
use regex::Regex;
use teo::prelude::Value;
pub use object_id::ObjectId;
pub use file::File;
pub use range::Range;
pub use enum_variant::EnumVariant;
pub use option_variant::OptionVariant;
use std::str::FromStr;
use bigdecimal::BigDecimal;
use pyo3::types::{PyBool, PyDate, PyDateTime, PyFloat, PyInt};
use chrono::prelude::{NaiveDate, Utc, DateTime};


use self::decimal::big_decimal_to_python_decimal;

pub fn teo_value_to_py_any<'p>(py: Python<'p>, value: &Value) -> PyResult<PyAny> {
    Ok(match value {
        Value::Null => ().into_py(py),
        Value::ObjectId(oid) => Ok(ObjectId { value: oid.clone() }.into_py(py)),
        Value::String(s) => Ok(s.into_py(py)),
        Value::Int(i) => Ok(i.into_py(py)),
        Value::Int64(i) => Ok(i.into_py(py)),
        Value::Float32(f) => Ok(f.into_py(py)),
        Value::Float(f) => Ok(f.into_py(py)),
        Value::Bool(b) => Ok(b.into_py(py)),
        Value::Date(d) => Ok(d.into_py(py)),
        Value::DateTime(d) => Ok(d.into_py(py)),
        Value::Decimal(b) => Ok(big_decimal_to_python_decimal(b, py)?),
        Value::Array(v) => {
            let list = PyList::empty(py);
            for value in v {
                list.append(teo_value_to_py_any(value, py)?)?;
            }
            Ok(list.into_py(py))
        },

        Value::Dictionary(m) => {
            let dict = PyDict::new(py);
            for (k, v) in m {
                dict.set_item(k, teo_value_to_py_any(v, py)?)?;
            }
            Ok(dict.into_py(py))
        },
        Value::Range(range) => {
            let instance = Range { value: range.clone() };
            Ok(instance.into_py(py))
        }
        Value::Tuple(tuple) => {
            Ok(PyTuple::new(py, tuple).into_py(py))
        }
        Value::EnumVariant(enum_variant) => {
            Ok(PyString::new(py, enum_variant.value.as_str()).into_py(py))
        }
        Value::OptionVariant(option_variant) => {
            let instance = OptionVariant { value: option_variant.clone() };
            Ok(instance.into_py(py))
        }
        Value::Regex(regex) => {
            let re = py.import("re")?;
            let compile = re.getattr("compile")?;
            let result = compile.call((regex.as_str(),), None)?;
            Ok(result)
        }
        Value::File(file) => {
            let instance = File::from(file);
            Ok(instance.into_py(py))
        }
        _ => Err(PyValueError::new_err("cannot convert Teo value to Python value")),
    })
}

pub fn py_any_to_teo_value(py: Python<'_>, object: &PyAny) -> PyResult<Value> {
    if object.is_none() {
        Ok(Value::Null)
    } else if object.is_instance_of::<ObjectId>()? {
        let object_id: ObjectId = object.extract()?;
        Ok(Value::ObjectId(object_id.value.clone()))
    } else if object.is_instance_of::<PyString>()? {
        let s: String = object.extract()?;
        Ok(Value::String(s))
    } else if object.is_instance_of::<PyBool>()? {
        let b: bool = object.extract()?;
        Ok(Value::Bool(b))
    } else if object.is_instance_of::<PyInt>()? {
        let i: i64 = object.extract()?;
        Ok(Value::Int64(i))
    } else if object.is_instance_of::<PyFloat>()? {
        let f: f64 = object.extract()?;
        Ok(Value::Float(f))
    } else if object.is_instance_of::<PyDate>()? {
        let d: NaiveDate = object.extract()?;
        Ok(Value::Date(d))
    } else if object.is_instance_of::<PyDateTime>()? {
        let d: DateTime<Utc> = object.extract()?;
        Ok(Value::DateTime(d))
    } else if object.is_instance_of::<PyList>()? {
        let v: Vec<&PyAny> = object.extract()?;
        let mut vec = vec![];
        for value in v {
            vec.push(py_any_to_teo_value(py, value)?);
        }
        Ok(Value::Vec(vec))
    } else if object.is_instance_of::<PyDict>()? {
        let m: IndexMap<String, &PyAny> = object.extract()?;
        let mut map: IndexMap<String, Value> = IndexMap::new();
        for (k, v) in m {
            map.insert(k, py_any_to_teo_value(py, v)?);
        }
        Ok(Value::Dictionary(map))
    } else if object.is_instance_of::<Range>()? {
        let range: Range = object.extract()?;
        Ok(Value::Range(range.value.clone()))
    } else if object.is_instance_of::<PyTuple>()? {
        let tuple: PyTuple = object.extract()?;
        let mut vec = vec![];
        for value in tuple.iter() {
            vec.push(py_any_to_teo_value(py, value)?);
        }
        Ok(Value::Tuple(vec))

    } else if object.is_instance_of::<File>()? {
        let file: File = object.extract()?;
        Ok(Value::File(file.into()))
    } else {
        let decimal_module = py.import("decimal")?;
        let decimal_class = decimal_module.getattr("Decimal")?;
        let re_module = py.import("re")?;
        let pattern_class = re_module.getattr("Pattern")?;
        if object.is_instance(decimal_class)? {
            let s: String = object.call_method0("__str__")?.extract()?;
            Ok(Value::Decimal(BigDecimal::from_str(&s).unwrap()))
        } else if object.is_instance(pattern_class)? {
            let pattern: &str = object.getattr("pattern")?;
            let r: Regex = Regex::new(pattern).unwrap();
            Ok(Value::Regex(r))
        }
         else {
            Err(PyValueError::new_err("Cannot convert Python object to Teo value."))
        }
    }
}
