use pyo3::{PyObject, PyResult, Python};
use teo::prelude::{model, transaction};
use super::{wrappers::{ModelObjectWrapper, TransactionCtxWrapper}, FetchDynamicClasses};

pub trait QueryDynamicClasses: FetchDynamicClasses {

    fn teo_model_object_to_py_model_object_object(&self, py: Python<'_>, teo_model_object: model::Object) -> PyResult<PyObject> {
        let model_name = teo_model_object.model().path().join(".");
        let model_object_class = self.object(&model_name)?.unwrap();
        let model_object = model_object_class.call_method1(py, "__new__", (model_object_class.as_any(),))?;
        model_object.setattr(py, "__teo_object__", ModelObjectWrapper::new(teo_model_object))?;
        Ok(model_object)
    }

    fn teo_transaction_ctx_to_py_ctx_object(&self, py: Python<'_>, transaction_ctx: transaction::Ctx, name: &str) -> PyResult<PyObject> {
        let ctx_class = self.ctx(name)?.unwrap();
        let ctx_object = ctx_class.call_method1(py, "__new__", (ctx_class.as_any(),))?;
        ctx_object.setattr(py, "__teo_transaction_ctx__", TransactionCtxWrapper::new(transaction_ctx))?;
        Ok(ctx_object)
    }
}