use anyhow::Result;
use serde::{Deserialize, Serialize};
use svix_bridge_types::{TransformerInput, TransformerOutput};

mod engine_quickjs;

pub use engine_quickjs::validate_script;

#[derive(Clone, Copy, PartialEq, Eq, Default, Deserialize, Serialize, Debug)]
#[serde(rename_all = "kebab-case")]
pub enum JsExecutor {
    #[default]
    QuickJs,
    Deno,
    Both,
}

const MAX_QJS_DURATION: std::time::Duration = std::time::Duration::from_millis(100);

pub async fn run_script(input: TransformerInput, script: String) -> Result<TransformerOutput> {
    engine_quickjs::run_script(input, script, MAX_QJS_DURATION).await
}
