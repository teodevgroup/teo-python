use pyo3::{pyclass, pymethods};
use teo::prelude::Namespace as TeoNamespace;

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

    pub fn namespace(&'static mut self, name: String) -> Option<Namespace> {
        self.teo_namespace.namespace_mut(name.as_str()).map(|n| Namespace { teo_namespace: n })
    }

    pub fn namespace_or_create(&'static mut self, name: String) -> Namespace {
        Namespace{ teo_namespace: self.teo_namespace.namespace_mut_or_create(name.as_str()) }
    }

    pub fn namespace_at_path(&'static mut self, path: Vec<&str>) -> Option<Namespace> {
        self.teo_namespace.namespace_mut_at_path(&path).map(|n| Namespace { teo_namespace: n })
    }

    pub fn namespace_or_create_at_path(&'static mut self, path: Vec<&str>) -> Namespace {
        Namespace{ teo_namespace: self.teo_namespace.namespace_mut_or_create_at_path(&path) }
    }
}