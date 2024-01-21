use pyo3::{pyclass, pymethods, PyObject, PyResult};
use teo::prelude::{handler::Group as TeoHandlerGroup, request, Response as TeoResponse};

use crate::{object::value::teo_value_to_js_any, request::Request, dynamic::js_ctx_object_from_teo_transaction_ctx, response::response_or_promise::ResponseOrPromise, utils::check_callable::check_callable};

#[pyclass]
pub struct HandlerGroup {
    pub(crate) teo_handler_group: &'static mut TeoHandlerGroup,
}

#[pymethods]
impl HandlerGroup {

    pub fn define_handler(&mut self, name: String, callback: PyObject) -> PyResult<()> {
        check_callable(callback)?;
        self.teo_handler_group.define_handler(name.as_str(), move |ctx: request::Ctx| async move {
            Python::with_gil(|py| {
                
            }).into_teo_result()?
        });
        Ok(())
        let response_unknown: ResponseOrPromise = tsfn_cloned.call_async(ctx).await.unwrap();
        Ok::<TeoResponse, teo::prelude::path::Error>(response_unknown.to_teo_response().await.unwrap())
        
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
}