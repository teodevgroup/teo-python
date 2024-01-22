use ::teo::prelude::Value;
use ::teo::prelude::Error;
use ::teo::prelude::Result;
use teo::prelude::pipeline::item::validator::Validity;

pub fn validate_result(value: Value) -> Result<Validity> {
    Ok(match value {
        Value::Null => Validity::Valid,
        Value::String(s) => Validity::Invalid(s),
        Value::Bool(b) => Validity::from(b),
        _ => return Err(Error::new("return value of validate callback is not expected")),
    })
}