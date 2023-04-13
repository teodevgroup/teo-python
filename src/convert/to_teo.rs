use std::any::Any;
use std::str::FromStr;
use pyo3::{IntoPy, Py, PyAny, PyObject, Python, PyResult, PyErr};
use ::teo::prelude::Value;
use pyo3::exceptions::PyValueError;
use bigdecimal::BigDecimal;
use pyo3::types::{PyBool, PyDate, PyDateTime, PyFloat, PyInt, PyString};
use chrono::prelude::{NaiveDate, Utc, DateTime};

fn big_decimal_to_python_decimal(d: BigDecimal, py: Python<'_>) -> PyResult<PyObject> {
    let s = d.to_string();
    let decimal_module = py.import("decimal")?;
    let decimal_class = decimal_module.getattr("Decimal")?;
    let decimal_object = decimal_class.call1((s,))?;
    Ok(decimal_object.into_py(py))
}

pub fn py_object_to_teo_value(object: &PyAny, py: Python<'_>) -> PyResult<Value> {
    if object.is_instance_of::<PyString>()? {
        let s: String = object.extract()?;
        Ok(Value::String(s))
    } else if object.is_instance_of::<PyBool>()? {
        let b: bool = object.extract()?;
        Ok(Value::Bool(b))
    } else if object.is_instance_of::<PyInt>()? {
        let i: i64 = object.extract()?;
        Ok(Value::I64(i))
    } else if object.is_instance_of::<PyFloat>()? {
        let f: f64 = object.extract()?;
        Ok(Value::F64(f))
    } else if object.is_instance_of::<PyDate>()? {
        let d: NaiveDate = object.extract()?;
        Ok(Value::Date(d))
    } else if object.is_instance_of::<PyDateTime>()? {
        let d: DateTime<Utc> = object.extract()?;
        Ok(Value::DateTime(d))
    } else {
        let decimal_module = py.import("decimal")?;
        let decimal_class = decimal_module.getattr("Decimal")?;
        if object.is_instance(decimal_class)? {
            let s: String = object.call_method0("__str__")?.extract()?;
            Ok(Value::Decimal(BigDecimal::from_str(&s).unwrap()))
        } else {
            Err(PyValueError::new_err("Cannot convert Python object to Teo value."))
        }
    }
}
