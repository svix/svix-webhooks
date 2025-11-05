use std::{io::Read, str::FromStr};

use anyhow::{Context, Error, Result};
use colored_json::{Color, ColorMode, ToColoredJson};
use serde::{de::DeserializeOwned, Serialize};

#[derive(Clone, Debug, Default, PartialEq)]
pub struct JsonOf<T>(T);

impl<T: DeserializeOwned> FromStr for JsonOf<T> {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "-" {
            let mut stdin = std::io::stdin().lock();
            let mut input = String::new();
            stdin
                .read_to_string(&mut input)
                .context("Error reading stdin for '-' argument")?;
            Ok(JsonOf(serde_json::from_str(&input)?))
        } else {
            Ok(JsonOf(serde_json::from_str(s)?))
        }
    }
}

impl<T> JsonOf<T> {
    pub fn into_inner(self) -> T {
        self.0
    }
}

pub fn print_json_output<T>(val: &T, color_mode: ColorMode) -> Result<()>
where
    T: Serialize,
{
    let styler = colored_json::Styler {
        integer_value: Color::Green.foreground(),
        float_value: Color::Green.foreground(),
        bool_value: Color::Yellow.foreground(),
        nil_value: Color::Magenta.foreground(),
        string_include_quotation: true,
        ..Default::default()
    };
    let s = serde_json::to_string_pretty(val)?.to_colored_json_with_styler(color_mode, styler)?;

    println!("{s}");
    Ok(())
}
