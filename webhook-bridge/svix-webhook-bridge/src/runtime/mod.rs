use anyhow::{Error, Result};
use std::{pin::Pin, rc::Rc, sync::Arc};

use deno_core::{
    anyhow,
    futures::FutureExt,
    resolve_import, serde_v8,
    v8::{self, Global, Value},
    FsModuleLoader, ModuleLoader, ModuleSource, ModuleSourceFuture, ModuleSpecifier, ModuleType,
    ResolutionKind,
};
use deno_runtime::permissions::PermissionsContainer;
use deno_runtime::{
    permissions::Permissions,
    worker::{MainWorker, WorkerOptions},
};
use svix_webhook_bridge_types::{JsObject, JsReturn};
use threadpool::ThreadPool;
use tokio::sync::{oneshot, Mutex};

/// This [`ModuleLoader`] implementation loads the configured script when loading the "file"
/// `virt:///user/script`, but otherwise loads things from disk
pub struct ConfiguredModuleLoader(pub String);

impl ModuleLoader for ConfiguredModuleLoader {
    fn resolve(
        &self,
        specifier: &str,
        referrer: &str,
        _kind: ResolutionKind,
    ) -> std::result::Result<ModuleSpecifier, Error> {
        Ok(resolve_import(specifier, referrer)?)
    }

    fn load(
        &self,
        module_specifier: &ModuleSpecifier,
        maybe_referrer: Option<&ModuleSpecifier>,
        is_dyn_import: bool,
    ) -> Pin<Box<ModuleSourceFuture>> {
        let module_specifier = module_specifier.clone();
        let code = self.0.clone();
        if Ok(module_specifier.clone()) == resolve_import("virt:///user/script", "") {
            async move {
                Ok(ModuleSource::new(
                    ModuleType::JavaScript,
                    code.into(),
                    &module_specifier,
                ))
            }
            .boxed_local()
        } else {
            FsModuleLoader.load(&module_specifier, maybe_referrer, is_dyn_import)
        }
    }
}

// NOTE: The worker is in a struct to try and get around requirements for Send with `async`, but I
// think it can be eliminated with a little effort.
struct Worker {
    pub worker: MainWorker,
}

impl Worker {
    pub async fn load_script(&mut self, script: String) -> Result<()> {
        let loader = ConfiguredModuleLoader(script.clone());
        let worker_main_module =
            loader.resolve("virt:///user/script", "", ResolutionKind::MainModule)?;

        let module_id = self
            .worker
            .js_runtime
            .load_main_module(&worker_main_module, Some(script.into()))
            .await?;

        let eval = self.worker.js_runtime.mod_evaluate(module_id);
        self.worker.js_runtime.run_event_loop(true).await?;
        eval.await??;
        Ok(())
    }

    pub fn run_script(&mut self, input: &serde_json::Value) -> Result<Global<Value>> {
        // This defines the global `input` variable
        self.worker
            .execute_script("bootstrap", define_global(input)?.into())?;

        // And this calls the  `handler` function in the main module
        let out = self.worker.execute_script(
            "run",
            "import('virt:///user/script').then(module => module.default(input));"
                .to_string()
                .into(),
        )?;
        Ok(out)
    }

    pub async fn resolve_value(&mut self, res: Global<Value>) -> Result<JsReturn> {
        let awaited = self.worker.js_runtime.resolve_value(res).await?;
        let scope = &mut self.worker.js_runtime.handle_scope();
        let local = v8::Local::new(scope, awaited);
        match serde_v8::from_v8::<JsObject>(scope, local) {
            Ok(v) => Ok(JsReturn::Object(v)),
            Err(serde_v8::Error::ExpectedObject(_)) => Ok(JsReturn::Invalid),
            Err(e) => Err(e)?,
        }
    }
}

#[derive(Clone, Debug)]
pub struct TpHandle(pub Arc<Mutex<ThreadPool>>);

impl TpHandle {
    pub fn new() -> Self {
        Self(Arc::new(Mutex::new(ThreadPool::default())))
    }

    pub async fn run_script(&self, input: serde_json::Value, script: String) -> Result<JsReturn> {
        let (tx, rx) = oneshot::channel();

        self.0.lock().await.execute(move || {
            let _ = tx.send(run_script_inner(&input, script));
        });

        rx.await?
    }
}

fn run_script_inner(input: &serde_json::Value, script: String) -> Result<JsReturn> {
    let mut worker = {
        let worker_main_module = "file:///does_not_exist".parse().unwrap();

        let worker_options = WorkerOptions {
            module_loader: Rc::new(ConfiguredModuleLoader(script.clone())),
            ..Default::default()
        };

        let worker_permissions = PermissionsContainer::new(Permissions::allow_all());

        Worker {
            worker: MainWorker::bootstrap_from_options(
                worker_main_module,
                worker_permissions,
                worker_options,
            ),
        }
    };

    smol::block_on(async { worker.load_script(script).await })?;
    let res = worker.run_script(input)?;
    smol::block_on(async { worker.resolve_value(res).await })
}

fn define_global(val: &serde_json::Value) -> Result<String> {
    Ok(format!(
        "Object.defineProperty(\
            globalThis,\
            'input',\
            {{  value: {}, writable:  true, enumerable: true, configurable: true }}\
        );",
        serde_json::to_string(val)?,
    ))
}

#[cfg(test)]
mod tests;
