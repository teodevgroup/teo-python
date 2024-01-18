pub mod object_id;
pub mod file;
pub mod range;
pub mod enum_variant;
pub mod option_variant;
pub mod decimal;

use pyo3::{Python, PyAny, PyResult, IntoPy, types::{PyList, PyDict}, exceptions::PyValueError};
use teo::prelude::Value;
pub use object_id::ObjectId;
pub use file::File;
pub use range::Range;
pub use enum_variant::EnumVariant;
pub use option_variant::OptionVariant;

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
            let instance = Range { value: range.clone() }.into_instance(*env)?;
            instance.as_object(*env).into_unknown()
        }
        Value::Tuple(tuple) => {
            let mut js_array = env.create_array_with_length(tuple.len())?;
            for (i, value) in tuple.iter().enumerate() {
                let v = teo_value_to_js_any(value, env)?;
                js_array.set_element(i as u32, &v)?;
            }
            js_array.into_unknown()
        }
        Value::EnumVariant(enum_variant) => {
            env.create_string(enum_variant.value.as_str())?.into_unknown()
        }
        Value::OptionVariant(option_variant) => {
            let instance = OptionVariant { value: option_variant.clone() }.into_instance(*env)?;
            instance.as_object(*env).into_unknown()
        }
        Value::Regex(regex) => {
            let global = env.get_global()?;
            let reg_exp_constructor: JsFunction = global.get_named_property("RegExp")?;
            let js_regex_str = env.create_string(regex.as_str())?;
            let js_regex = reg_exp_constructor.new_instance(&[js_regex_str])?;
            js_regex.into_unknown()
        }
        Value::File(file) => {
            let instance = File::from(file);
            instance.into_instance(*env)?.as_object(*env).into_unknown()
        }
        _ => Err(PyValueError::new_err("cannot convert Teo value to Python value")),
    })
}
