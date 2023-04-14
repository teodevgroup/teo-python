use ::teo::core::teon::Value;
use ::teo::core::pipeline::items::function::validate::Validity;
use ::teo::core::result::Result;
use ::teo::prelude::Error;

pub fn validate_result(value: Value) -> Result<Validity> {
    Ok(match value {
        Value::Null => Validity::Valid,
        Value::String(s) => Validity::Invalid(s),
        Value::Bool(b) => Validity::from(b),
        _ => return Err(Error::custom_validation_error("Validate callback's return value is not expected.")),
    })
}