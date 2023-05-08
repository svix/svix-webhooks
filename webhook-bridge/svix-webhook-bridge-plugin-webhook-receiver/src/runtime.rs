//! N.b. this module is not currently attached to the project.
//! Retained as-is from the original webhook-ingester for now since it seems adaptable for the
//! upcoming "transformations" work.
use std::{pin::Pin, rc::Rc, sync::Arc};

use anyhow::Result;
use deno_core::{
    futures::FutureExt,
    resolve_import, serde_v8,
    v8::{self, Global, Value},
    FsModuleLoader, ModuleLoader, ModuleSource, ModuleSourceFuture, ModuleSpecifier, ModuleType,
};
use deno_runtime::{
    permissions::Permissions,
    worker::{MainWorker, WorkerOptions},
};
use threadpool::ThreadPool;
use tokio::sync::{oneshot, Mutex};

use crate::types::{SerializableRequest, Unvalidated};

/// This [`ModuleLoader`] implementation loads the configured script when loading the "file"
/// `virt:///user/script`, but otherwise loads things from disk
pub struct ConfiguredModuleLoader(pub String);

impl ModuleLoader for ConfiguredModuleLoader {
    fn resolve(&self, specifier: &str, referrer: &str, _is_main: bool) -> Result<ModuleSpecifier> {
        Ok(resolve_import(specifier, referrer)?)
    }

    fn load(
        &self,
        module_specifier: &ModuleSpecifier,
        _maybe_referrer: Option<ModuleSpecifier>,
        _is_dynamic: bool,
    ) -> Pin<Box<ModuleSourceFuture>> {
        let module_specifier = module_specifier.clone();
        let code = self.0.clone();

        if Ok(module_specifier.clone()) == resolve_import("virt:///user/script", "") {
            async move {
                Ok(ModuleSource {
                    code: Box::from(code.as_bytes()),
                    module_type: ModuleType::JavaScript,
                    module_url_specified: module_specifier.to_string(),
                    module_url_found: module_specifier.to_string(),
                })
            }
            .boxed_local()
        } else {
            FsModuleLoader.load(&module_specifier, _maybe_referrer, _is_dynamic)
        }
    }
}

/// In the context of this service, the only valid return value of the exported function run by the
/// Deno runtime is a `bool` value. Any other values are invalid and should become an error in  the
/// [`VerificationMethod`] implementation.
pub enum JsReturn {
    Bool(bool),
    Invalid,
}

// NOTE: The worker is in a struct to try and get around requirements for Send with `async`, but I
// think it can be eliminated with a little effort.
struct Worker {
    pub worker: MainWorker,
}

impl Worker {
    pub async fn load_script(&mut self, script: String) -> Result<()> {
        let loader = ConfiguredModuleLoader(script.clone());
        let worker_main_module = loader.resolve("virt:///user/script", "", true)?;

        let module_id = self
            .worker
            .js_runtime
            .load_main_module(&worker_main_module, Some(script))
            .await?;

        let eval = self.worker.js_runtime.mod_evaluate(module_id);
        self.worker.js_runtime.run_event_loop(true).await?;
        eval.await??;

        Ok(())
    }

    pub fn run_script(&mut self, req: SerializableRequest<Unvalidated>) -> Result<Global<Value>> {
        // This defines the global `input` variable
        self.worker
            .execute_script("bootstrap", &define_global(&req)?)?;

        // And this calls the  `handler` function in the main module
        let out = self.worker.execute_script(
            "run",
            "import('virt:///user/script').then(module => module.default(input));",
        )?;

        Ok(out)
    }

    pub async fn resolve_value(&mut self, res: Global<Value>) -> Result<JsReturn> {
        let awaited = self.worker.js_runtime.resolve_value(res).await?;

        let scope = &mut self.worker.js_runtime.handle_scope();
        let local = v8::Local::new(scope, awaited);

        match serde_v8::from_v8::<bool>(scope, local) {
            Ok(b) => Ok(JsReturn::Bool(b)),
            Err(e) if e == deno_core::serde_v8::Error::ExpectedBoolean => Ok(JsReturn::Invalid),
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

    pub async fn run_script(
        &self,
        req: SerializableRequest<Unvalidated>,
        script: String,
    ) -> Result<JsReturn> {
        let (tx, rx) = oneshot::channel();

        self.0.lock().await.execute(move || {
            let _ = tx.send(run_script_inner(req, script));
        });

        rx.await?
    }
}

fn run_script_inner(req: SerializableRequest<Unvalidated>, script: String) -> Result<JsReturn> {
    let mut worker = {
        let worker_main_module = deno_core::resolve_path("file://dne").unwrap();

        let worker_options = WorkerOptions {
            module_loader: Rc::new(ConfiguredModuleLoader(script.clone())),
            ..Default::default()
        };

        let worker_permissions = Permissions::allow_all();

        Worker {
            worker: MainWorker::bootstrap_from_options(
                worker_main_module,
                worker_permissions,
                worker_options,
            ),
        }
    };

    smol::block_on(async { worker.load_script(script).await })?;
    let res = worker.run_script(req)?;
    smol::block_on(async { worker.resolve_value(res).await })
}

fn define_global(req: &SerializableRequest<Unvalidated>) -> Result<String> {
    Ok(format!(
        "Object.defineProperty(\
            globalThis,\
            'input',\
            {{  value: {}, writable:  true, enumerable: true, configurable: true }}\
        );",
        serde_json::to_string(req)?,
    ))
}
