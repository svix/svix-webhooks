use std::num::NonZeroUsize;

use anyhow::Result;
use deadpool::unmanaged::Pool;
use serde::{Deserialize, Serialize};
use svix_bridge_types::{TransformerInput, TransformerOutput};
use tokio::sync::oneshot;

mod engine_deno;
mod engine_quickjs;

#[derive(Clone, Copy, PartialEq, Eq, Default, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum JsExecutor {
    #[default]
    Deno,
    QuickJs,
    Both,
}

impl JsExecutor {
    fn run_deno(&self) -> bool {
        matches!(self, Self::Deno | Self::Both)
    }

    fn run_quickjs(&self) -> bool {
        matches!(self, Self::QuickJs | Self::Both)
    }
}

struct DenoExecutor {
    tx: std::sync::mpsc::Sender<Job>,
    _handle: std::thread::JoinHandle<()>,
}

impl Default for DenoExecutor {
    fn default() -> Self {
        let (tx, rx) = std::sync::mpsc::channel::<Job>();
        let _handle = std::thread::spawn(move || {
            let mut runtime = engine_deno::build_runtime();
            for Job { input, script, cb } in rx {
                let ret = engine_deno::run_script_inner(&mut runtime, input, script);
                if cb.send(ret).is_err() {
                    tracing::error!("failed to send script output to caller");
                }
            }
        });
        Self { tx, _handle }
    }
}

type Callback = oneshot::Sender<Result<TransformerOutput>>;

struct Job {
    input: TransformerInput,
    script: String,
    cb: Callback,
}

impl DenoExecutor {
    async fn execute(
        &mut self,
        input: TransformerInput,
        script: String,
    ) -> Result<TransformerOutput> {
        let (tx, rx) = oneshot::channel();
        self.tx.send(Job {
            input,
            script,
            cb: tx,
        })?;
        rx.await?
    }
}

#[derive(Clone)]
pub struct JsPooler {
    deno_executors: Pool<DenoExecutor>,
    backend: JsExecutor,
}

const MAX_QJS_DURATION: std::time::Duration = std::time::Duration::from_millis(100);

impl JsPooler {
    pub fn new(pool_size: NonZeroUsize, backend: JsExecutor) -> Self {
        let pool_size = pool_size.get();
        let mut items = Vec::with_capacity(pool_size);
        for _ in 0..pool_size {
            items.push(DenoExecutor::default());
        }
        Self {
            deno_executors: Pool::from(items),
            backend,
        }
    }

    pub async fn run_script(
        &self,
        input: TransformerInput,
        script: String,
    ) -> Result<TransformerOutput> {
        let deno_output = if self.backend.run_deno() {
            let pool = self.deno_executors.clone();
            let mut executor = pool.get().await;

            Some(
                executor
                    .as_mut()
                    .map_err(|e| anyhow::anyhow!("{e:?}"))?
                    .execute(input.clone(), script.clone())
                    .await,
            )
        } else {
            None
        };
        let quickjs_output = if self.backend.run_quickjs() {
            Some(engine_quickjs::run_script(input, script, MAX_QJS_DURATION).await)
        } else {
            None
        };

        match (deno_output, quickjs_output) {
            (Some(lhs), Some(rhs)) => compare(lhs, rhs),
            (Some(lhs), None) => lhs,
            (None, Some(rhs)) => rhs,
            (None, None) => anyhow::bail!("no JS executor backend configured"),
        }
    }
}

fn compare(
    deno: Result<TransformerOutput>,
    qjs: Result<TransformerOutput>,
) -> Result<TransformerOutput> {
    match (deno, qjs) {
        (Ok(d), Ok(q)) => {
            if d == q {
                Ok(q)
            } else {
                tracing::warn!(deno_output=?d, quickjs_output=?q, "deno and qjs disagreed; returning deno");
                Ok(d)
            }
        }
        (Ok(d), Err(q)) => {
            tracing::warn!(quickjs_error = %q, "qjs returned an error, but deno succeeded");
            Ok(d)
        }
        (Err(d), Ok(q)) => {
            tracing::warn!(deno_error = %d, "deno returned an error, but qjs succeeded");
            Ok(q)
        }
        (Err(d), Err(q)) => {
            tracing::warn!(deno_err = %d, quickjs_err = %q, "both deno and quickjs returned errors");
            Err(q)
        }
    }
}

pub(super) fn validate_script(src: &str) -> Result<()> {
    engine_deno::validate_script(src)
}
