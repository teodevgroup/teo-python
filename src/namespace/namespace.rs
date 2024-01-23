use pyo3::{pyclass, pymethods, PyResult, PyObject, Python, Py, PyErr};
use teo::prelude::{Namespace as TeoNamespace, Model as TeoModel, model::Field as TeoField, model::Relation as TeoRelation, model::Property as TeoProperty, Enum as TeoEnum, Member as TeoEnumMember};

use crate::{utils::check_callable::check_callable, object::arguments::teo_args_to_py_args, model::{model::Model, field::field::Field, relation::relation::Relation, property::property::Property}, result::IntoTeoResult, r#enum::{r#enum::Enum, member::member::EnumMember}};

#[pyclass]
pub struct Namespace {
    pub(crate) teo_namespace: &'static mut TeoNamespace,
}

#[pymethods]
impl Namespace {

    pub fn is_main(&self) -> bool {
        self.teo_namespace.is_main()
    }

    pub fn is_std(&self) -> bool {
        self.teo_namespace.is_std()
    }

    pub fn path(&self) -> Vec<&str> {
        self.teo_namespace.path()
    }

    pub fn namespace(&mut self, name: String) -> Option<Namespace> {
        let static_self: &'static mut Namespace = unsafe { &mut *(self as * const Namespace as * mut Namespace) };
        static_self.teo_namespace.namespace_mut(name.as_str()).map(|n| Namespace { teo_namespace: n })
    }

    pub fn namespace_or_create(&mut self, name: String) -> Namespace {
        let static_self: &'static mut Namespace = unsafe { &mut *(self as * const Namespace as * mut Namespace) };
        Namespace { teo_namespace: static_self.teo_namespace.namespace_mut_or_create(name.as_str()) }
    }

    pub fn namespace_at_path(&mut self, path: Vec<&str>) -> Option<Namespace> {
        let static_self: &'static mut Namespace = unsafe { &mut *(self as * const Namespace as * mut Namespace) };
        static_self.teo_namespace.namespace_mut_at_path(&path).map(|n| Namespace { teo_namespace: n })
    }

    pub fn namespace_or_create_at_path(&mut self, path: Vec<&str>) -> Namespace {
        let static_self: &'static mut Namespace = unsafe { &mut *(self as * const Namespace as * mut Namespace) };
        Namespace { teo_namespace: static_self.teo_namespace.namespace_mut_or_create_at_path(&path) }
    }

    pub fn define_model_decorator(&mut self, py: Python<'_>, name: &str, callback: PyObject) -> PyResult<()> {
        check_callable(callback.as_ref(py))?;
        let callback_owned = &*Box::leak(Box::new(Py::from(callback)));
        self.teo_namespace.define_model_decorator(name, |arguments, model| {
            Python::with_gil(|py| {
                let arguments = teo_args_to_py_args(py, &arguments)?;
                let static_model: &'static mut TeoModel = unsafe { &mut *(model as * mut TeoModel) };
                let model_wrapped = Model {
                    teo_model: static_model
                };
                callback_owned.call1(py, (arguments, model_wrapped))?;
                Ok::<(), PyErr>(())
            }).into_teo_result()?;
            Ok(())
        });
        Ok(())
    }

    pub fn define_model_field_decorator(&mut self, py: Python<'_>, name: &str, callback: PyObject) -> PyResult<()> {
        check_callable(callback.as_ref(py))?;
        let callback_owned = &*Box::leak(Box::new(Py::from(callback)));
        self.teo_namespace.define_model_field_decorator(name, |arguments, field| {
            Python::with_gil(|py| {
                let arguments = teo_args_to_py_args(py, &arguments)?;
                let static_field: &'static mut TeoField = unsafe { &mut *(field as * mut TeoField) };
                let model_wrapped = Field {
                    teo_field: static_field
                };
                callback_owned.call1(py, (arguments, model_wrapped))?;
                Ok::<(), PyErr>(())
            }).into_teo_result()?;
            Ok(())
        });
        Ok(())
    }

    pub fn define_model_relation_decorator(&mut self, py: Python<'_>, name: &str, callback: PyObject) -> PyResult<()> {
        check_callable(callback.as_ref(py))?;
        let callback_owned = &*Box::leak(Box::new(Py::from(callback)));
        self.teo_namespace.define_model_relation_decorator(name, |arguments, relation| {
            Python::with_gil(|py| {
                let arguments = teo_args_to_py_args(py, &arguments)?;
                let static_relation: &'static mut TeoRelation = unsafe { &mut *(relation as * mut TeoRelation) };
                let model_wrapped = Relation {
                    teo_relation: static_relation
                };
                callback_owned.call1(py, (arguments, model_wrapped))?;
                Ok::<(), PyErr>(())
            }).into_teo_result()?;
            Ok(())
        });
        Ok(())
    }

    pub fn define_model_property_decorator(&mut self, py: Python<'_>, name: &str, callback: PyObject) -> PyResult<()> {
        check_callable(callback.as_ref(py))?;
        let callback_owned = &*Box::leak(Box::new(Py::from(callback)));
        self.teo_namespace.define_model_property_decorator(name, |arguments, property| {
            Python::with_gil(|py| {
                let arguments = teo_args_to_py_args(py, &arguments)?;
                let static_property: &'static mut TeoProperty = unsafe { &mut *(property as * mut TeoProperty) };
                let model_wrapped = Property {
                    teo_property: static_property
                };
                callback_owned.call1(py, (arguments, model_wrapped))?;
                Ok::<(), PyErr>(())
            }).into_teo_result()?;
            Ok(())
        });
        Ok(())
    }

    pub fn define_enum_decorator(&mut self, py: Python<'_>, name: &str, callback: PyObject) -> PyResult<()> {
        check_callable(callback.as_ref(py))?;
        let callback_owned = &*Box::leak(Box::new(Py::from(callback)));
        self.teo_namespace.define_enum_decorator(name, |arguments, teo_enum: &mut TeoEnum| {
            Python::with_gil(|py| {
                let arguments = teo_args_to_py_args(py, &arguments)?;
                let static_teo_enum: &'static mut TeoEnum = unsafe { &mut *(teo_enum as * mut TeoEnum) };
                let model_wrapped = Enum {
                    teo_enum: static_teo_enum
                };
                callback_owned.call1(py, (arguments, model_wrapped))?;
                Ok::<(), PyErr>(())
            }).into_teo_result()?;
            Ok(())
        });
        Ok(())
    }

    pub fn define_enum_member_decorator(&mut self, py: Python<'_>, name: &str, callback: PyObject) -> PyResult<()> {
        check_callable(callback.as_ref(py))?;
        let callback_owned = &*Box::leak(Box::new(Py::from(callback)));
        self.teo_namespace.define_enum_member_decorator(name, |arguments, teo_enum: &mut TeoEnumMember| {
            Python::with_gil(|py| {
                let arguments = teo_args_to_py_args(py, &arguments)?;
                let static_teo_enum: &'static mut TeoEnumMember = unsafe { &mut *(teo_enum as * mut TeoEnumMember) };
                let model_wrapped = EnumMember {
                    teo_enum_member: static_teo_enum
                };
                callback_owned.call1(py, (arguments, model_wrapped))?;
                Ok::<(), PyErr>(())
            }).into_teo_result()?;
            Ok(())
        });
        Ok(())
    }
}