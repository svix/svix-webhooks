use serde::{
    Deserialize,
    Serialize,
};

/// Validation errors have their own schema to provide context
/// for invalid requests eg. mismatched types and out of bounds values. There
/// may be any number of these per 422 UNPROCESSABLE ENTITY error.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ValidationError {
    /// The location as a [`Vec`] of [`String`]s -- often in the form
    /// `[\"body\", \"field_name\"]`, `[\"query\", \"field_name\"]`, etc. They
    /// may, however, be arbitrarily deep.
    pub loc: Vec<String>,
    /// The message accompanying the validation error item.
    pub msg: String,
    /// The type of error, often \"type_error\" or \"value_error\", but
    /// sometimes with more context like as \"value_error.number.not_ge\"
    pub r#type: String,
}

impl ValidationError {
    /// Validation errors have their own schema to provide context for invalid
    /// requests eg. mismatched types and out of bounds values. There may be any
    /// number of these per 422 UNPROCESSABLE ENTITY error.
    pub fn new(
        loc: Vec<String>,
        msg: String,
        r#type: String,
    ) -> ValidationError {
        ValidationError {
            loc,
            msg,
            r#type,
        }
    }
}
