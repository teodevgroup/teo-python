use pyo3::{IntoPy, Py, PyAny, PyObject, Python, PyResult, PyErr};
use ::teo::prelude::Value;
use pyo3::exceptions::PyValueError;
use bigdecimal::BigDecimal;
use pyo3::types::{PyDict, PyList};

fn big_decimal_to_python_decimal(d: BigDecimal, py: Python<'_>) -> PyResult<PyObject> {
    let s = d.to_string();
    let decimal_module = py.import("decimal")?;
    let decimal_class = decimal_module.getattr("Decimal")?;
    let decimal_object = decimal_class.call1((s,))?;
    Ok(decimal_object.into_py(py))
}

pub fn teo_value_to_py_object(value: Value, py: Python<'_>) -> PyResult<PyObject> {
    match value {
        Value::Null => Ok(().into_py(py)),
        Value::ObjectId(oid) => Ok(oid.to_string().into_py(py)),
        Value::String(s) => Ok(s.into_py(py)),
        Value::I32(i) => Ok(i.into_py(py)),
        Value::I64(i) => Ok(i.into_py(py)),
        Value::F32(f) => Ok(f.into_py(py)),
        Value::F64(f) => Ok(f.into_py(py)),
        Value::Bool(b) => Ok(b.into_py(py)),
        Value::Date(d) => Ok(d.into_py(py)),
        Value::DateTime(d) => Ok(d.into_py(py)),
        Value::Decimal(b) => Ok(big_decimal_to_python_decimal(b, py)?),
        Value::Vec(v) => {
            let list = PyList::empty(py);
            for value in v {
                list.append(teo_value_to_py_object(value, py)?)?;
            }
            Ok(list.into_py(py))
        },
        Value::HashMap(m) => {
            let dict = PyDict::new(py);
            for (k, v) in m {
                dict.set_item(k, teo_value_to_py_object(v, py)?)?;
            }
            Ok(dict.into_py(py))
        },
        Value::IndexMap(m) => {
            let dict = PyDict::new(py);
            for (k, v) in m {
                dict.set_item(k, teo_value_to_py_object(v, py)?)?;
            }
            Ok(dict.into_py(py))
        },
        Value::BTreeMap(m) => {
            let dict = PyDict::new(py);
            for (k, v) in m {
                dict.set_item(k, teo_value_to_py_object(v, py)?)?;
            }
            Ok(dict.into_py(py))
        },
        _ => Err(PyValueError::new_err("Cannot convert Teo type to Python type.")),
    }
}