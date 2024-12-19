use anyhow::{Error, Result};
use colored_json::ColorMode;
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::str::FromStr;

#[derive(Clone, Debug, PartialEq)]
pub struct JsonOf<T: DeserializeOwned>(T);

impl<T: DeserializeOwned> FromStr for JsonOf<T> {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(JsonOf(serde_json::from_str(s)?))
    }
}

impl<T: DeserializeOwned> JsonOf<T> {
    pub fn into_inner(self) -> T {
        self.0
    }
}

pub fn print_json_output<T>(val: &T, color_mode: ColorMode) -> Result<()>
where
    T: Serialize,
{
    // FIXME: factor the writer out? Will that help with testing?
    let mut writer = std::io::stdout().lock();
    colored_json::write_colored_json_with_mode(val, &mut writer, color_mode)?;
    Ok(())
}
