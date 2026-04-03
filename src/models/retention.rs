// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Retention {
    #[serde(
        rename = "period_ms",
        skip_serializing_if = "Option::is_none",
        with = "crate::duration_ms_serde::optional"
    )]
    pub period: Option<std::time::Duration>,
}

impl Retention {
    pub fn new() -> Self {
        Self { period: None }
    }

    pub fn with_period(mut self, value: impl Into<Option<std::time::Duration>>) -> Self {
        self.period = value.into();
        self
    }
}
