use pyo3::{IntoPy, Py, PyAny, PyObject, Python, PyResult, PyErr};
use ::teo::prelude::Value;
use pyo3::exceptions::PyValueError;
use bigdecimal::BigDecimal;

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
        Value::Bool(b) => Ok(b.into_py(py)),
        Value::Date(d) => Ok(d.into_py(py)),
        Value::DateTime(d) => Ok(d.into_py(py)),
        Value::Decimal(b) => Ok(big_decimal_to_python_decimal(b, py)?),
        _ => Err(PyValueError::new_err("Cannot convert Teo type to Python type.")),
    }
}