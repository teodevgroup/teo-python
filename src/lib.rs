pub mod utils;
pub mod app;
pub mod namespace;
pub mod dynamic;
pub mod object;
pub mod r#enum;
pub mod model;
pub mod handler;
pub mod request;
pub mod response;

use pyo3::prelude::*;
use ::teo::prelude::serve_static_files as teo_serve_static_files;
use request::{Expiration, Request};
use response::{Response, header_map::ReadWriteHeaderMap};
use crate::app::app::App;
use crate::namespace::namespace::Namespace;
use crate::r#enum::r#enum::Enum;
use crate::r#enum::member::member::EnumMember;
use crate::model::model::Model;
use crate::model::field::field::Field;
use crate::model::relation::relation::Relation;
use crate::model::property::property::Property;
use crate::handler::group::HandlerGroup;
use crate::request::HandlerMatch;
use crate::request::ctx::RequestCtx;
use crate::request::Cookie;
use crate::object::value::ObjectId;
use crate::object::value::OptionVariant;
use crate::object::value::File;
use crate::object::value::Range;
use crate::object::pipeline::Pipeline;
use crate::object::interface_enum_variant::InterfaceEnumVariant;

#[pymodule]
fn teo(py: Python, m: &PyModule) -> PyResult<()> {
    py.run(r#"
global teo_wrap_builtin
def teo_wrap_builtin(cls, name, callable):
    def wrapped(self, *args, **kwargs):
        return callable(self, *args, **kwargs)
    setattr(cls, name, wrapped)

global teo_wrap_async
def teo_wrap_async(callable):
    async def wrapped(self, *args, **kwargs):
        return await callable(self, *args, **kwargs)
    return wrapped
    "#, None, None)?;
    #[pyfunction]
    fn serve_static_files(base: &str, path: &str, py: Python<'_>) -> PyResult<Response> {
        let teo_response = teo_serve_static_files(base, path)?;
        Ok(Response { teo_response })
    }
    m.add_function(wrap_pyfunction!(serve_static_files, m)?)?;
    m.add_class::<App>()?;
    m.add_class::<Namespace>()?;
    m.add_class::<HandlerGroup>()?;
    m.add_class::<Model>()?;
    m.add_class::<Field>()?;
    m.add_class::<Relation>()?;
    m.add_class::<Property>()?;
    m.add_class::<Enum>()?;
    m.add_class::<EnumMember>()?;
    m.add_class::<Response>()?;
    m.add_class::<Request>()?;
    m.add_class::<Cookie>()?;
    m.add_class::<Expiration>()?;
    m.add_class::<ReadWriteHeaderMap>()?;
    m.add_class::<HandlerMatch>()?;
    m.add_class::<RequestCtx>()?;
    m.add_class::<ObjectId>()?;
    m.add_class::<Range>()?;
    m.add_class::<OptionVariant>()?;
    m.add_class::<File>()?;
    m.add_class::<InterfaceEnumVariant>()?;
    m.add_class::<Pipeline>()?;
    Ok(())
}
