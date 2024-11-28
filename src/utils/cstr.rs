use std::ffi::{CStr, CString};
use teo::prelude::Result;

pub(crate) fn static_cstr(base: &str) -> Result<&'static CStr> {
    Ok(unsafe { &*Box::into_raw(Box::new(CString::new(base)?)) }.as_c_str())
}