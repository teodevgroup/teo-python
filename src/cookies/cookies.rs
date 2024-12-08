use pyo3::{pyclass, pymethods};
use teo::prelude::cookies::Cookies as OriginalCookies;
use super::Cookie;

#[pyclass]
#[derive(Clone)]
pub struct Cookies {
    original: OriginalCookies,
}

impl From<OriginalCookies> for Cookies {
    fn from(original: OriginalCookies) -> Self {
        Self { original }
    }
}

impl Cookies {
    pub(crate) fn original(&self) -> &OriginalCookies {
        &self.original
    }
}

#[pymethods]
impl Cookies {
    
    #[new]
    #[pyo3(signature = (cookies = None))]
    pub fn new(cookies: Option<Vec<Cookie>>) -> Self {
        if let Some(cookies) = cookies {
            let original_cookies = OriginalCookies::new();
            original_cookies.set_entries(cookies.into_iter().map(|cookie| cookie.original().clone()).collect());
            Cookies {
                original: original_cookies,
            }
        } else {
            Cookies {
                original: OriginalCookies::new(),
            }    
        }
    }

    pub fn __getitem__(&self, key: &str) -> Option<Cookie> {
        self.original.get(key).map(|cookie| Cookie::from(cookie.clone()))
    }

    pub fn __hasitem__(&self, key: &str) -> bool {
        self.original.has(key)
    }

    pub fn append(&self, cookie: Cookie) {
        self.original.push(cookie.original().clone())
    }

    pub fn clear(&self) {
        self.original.clear()
    }

    pub fn __len__(&self) -> usize {
        self.original.len()
    }

    pub fn __iter__(&self) -> CookiesIter {
        CookiesIter {
            original: self.original.entries().into_iter().map(|entry| Cookie::from(entry)).collect(),
            index: 0
        }
    }
}

#[pyclass]
pub struct CookiesIter {
    pub(super) original: Vec<Cookie>,
    pub(super) index: usize,
}

#[pymethods]
impl CookiesIter {

    fn __next__(&mut self) -> Option<Cookie> {
        if self.index >= self.original.len() {
            return None;
        }
        let result = self.original.get(self.index)?;
        self.index += 1;
        Some(result.clone())
    }
}