
fn transform(&self, _py: Python, name: &str, callback: &PyAny) -> PyResult<()> {
    check_callable(callback)?;
    let mut_builder = self.app_builder.as_ref().to_mut();
    let callback_owned = Box::leak(Box::new(Py::from(callback)));
    mut_builder.transform(name, |value: Value| async {
        Python::with_gil(|py| {
            let callback = callback_owned.as_ref(py);
            let py_object = teo_value_to_py_object(value, py)?;
            let transformed_py = callback.call1((py_object,))?;
            let transformed_py_awaited = await_coroutine_if_needed(transformed_py, py)?;
            let transformed_py_awaited_ref = transformed_py_awaited.as_ref(py);
            let transformed = py_object_to_teo_value(transformed_py_awaited_ref, py)?;
            Ok(transformed)
        }).into_teo_result()
    });
    Ok(())
}

fn validate(&self, _py: Python, name: &str, callback: &PyAny) -> PyResult<()> {
    check_callable(callback)?;
    let mut_builder = self.app_builder.as_ref().to_mut();
    let callback_owned = Box::leak(Box::new(Py::from(callback)));
    mut_builder.validate(name, |value: Value| async {
        Python::with_gil(|py| {
            let callback = callback_owned.as_ref(py);
            let py_object = teo_value_to_py_object(value, py)?;
            let transformed_py = callback.call1((py_object,))?;
            let transformed_py_awaited = await_coroutine_if_needed(transformed_py, py)?;
            let transformed_py_awaited_ref = transformed_py_awaited.as_ref(py);
            let transformed = py_object_to_teo_value(transformed_py_awaited_ref, py)?;
            Ok(validate_result(transformed))
        }).into_teo_result()?
    });
    Ok(())
}

fn callback(&self, _py: Python, name: &str, callback: &PyAny) -> PyResult<()> {
    check_callable(callback)?;
    let mut_builder = self.app_builder.as_ref().to_mut();
    let callback_owned = Box::leak(Box::new(Py::from(callback)));
    mut_builder.callback(name, |value: Value| async {
        Python::with_gil(|py| {
            let callback = callback_owned.as_ref(py);
            let py_object = teo_value_to_py_object(value, py)?;
            let transformed_py = callback.call1((py_object,))?;
            let _ = await_coroutine_if_needed(transformed_py, py)?;
            Ok(())
        }).into_teo_result()
    });
    Ok(())
}

fn compare(&self, _py: Python, name: &str, callback: &PyAny) -> PyResult<()> {
    check_callable(callback)?;
    let mut_builder = self.app_builder.as_ref().to_mut();
    let callback_owned = Box::leak(Box::new(Py::from(callback)));
    mut_builder.compare(name, |old: Value, new: Value| async {
        Python::with_gil(|py| {
            let callback = callback_owned.as_ref(py);
            let py_object_old = teo_value_to_py_object(old, py)?;
            let py_object_new = teo_value_to_py_object(new, py)?;
            let transformed_py = callback.call1((py_object_old,py_object_new))?;
            let transformed_py_awaited = await_coroutine_if_needed(transformed_py, py)?;
            let transformed_py_awaited_ref = transformed_py_awaited.as_ref(py);
            let transformed = py_object_to_teo_value(transformed_py_awaited_ref, py)?;
            Ok(validate_result(transformed))
        }).into_teo_result()?
    });
    Ok(())
}