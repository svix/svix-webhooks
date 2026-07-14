use std::{io::Read, str::FromStr};

use anyhow::{Context, Error, Result};
use colored_json::{Color, ColorMode, ColoredFormatter, PrettyFormatter};
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

fn serialize_json<T, F>(val: &T, formatter: F) -> Result<()>
where
    T: Serialize,
    F: serde_json::ser::Formatter,
{
    let stdout = std::io::stdout().lock();
    let mut serializer = serde_json::Serializer::with_formatter(stdout, formatter);
    match val.serialize(&mut serializer) {
        Ok(_) => Ok(()),
        Err(e) if matches!(e.io_error_kind(), Some(std::io::ErrorKind::BrokenPipe)) => Ok(()),
        Err(e) => Err(e.into()),
    }
}

pub fn print_json_output<T>(val: &T, color_mode: ColorMode) -> Result<()>
where
    T: Serialize,
{
    if color_mode.use_color() {
        let styler = colored_json::Styler {
            integer_value: Color::Green.foreground(),
            float_value: Color::Green.foreground(),
            bool_value: Color::Yellow.foreground(),
            nil_value: Color::Magenta.foreground(),
            string_include_quotation: true,
            ..Default::default()
        };
        serialize_json(
            val,
            ColoredFormatter::with_styler(PrettyFormatter::new(), styler),
        )
    } else {
        serialize_json(val, PrettyFormatter::new())
    }
}
