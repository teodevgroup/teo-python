use bigdecimal::BigDecimal;
use pyo3::{types::PyAnyMethods, IntoPy, PyObject, PyResult, Python};

pub fn big_decimal_to_python_decimal(d: BigDecimal, py: Python<'_>) -> PyResult<PyObject> {
    let s = d.to_string();
    let decimal_module = py.import_bound("decimal")?;
    let decimal_class = decimal_module.getattr("Decimal")?;
    let decimal_object = decimal_class.call1((s,))?;
    Ok(decimal_object.into_py(py))
}
