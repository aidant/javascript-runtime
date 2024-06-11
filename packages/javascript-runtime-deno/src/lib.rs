use anyhow::anyhow;
use deno_core::{FsModuleLoader, ModuleSpecifier};
use deno_runtime::{
    permissions::PermissionsContainer,
    worker::{MainWorker, WorkerOptions},
};
use event::{DispatchEvent, MessageEventInit};
use op_javascript_runtime::BridgeContainer;
use std::{
    collections::HashMap,
    path::Path,
    rc::Rc,
    str::FromStr,
    sync::Arc,
    thread::{self, JoinHandle},
};
use tokio::sync::{
    broadcast,
    oneshot::{self, Sender},
    Mutex, RwLock,
};
use uuid::Uuid;

mod event;
mod op_javascript_runtime;

pub trait JavaScriptRuntime: Send + Sync {
    fn start(&self, id: String, specifier: String) -> Result<(), JavaScriptRuntimeError>;
    fn close(&self, id: String) -> Result<(), JavaScriptRuntimeError>;
    fn post_message(&self, id: String, message: String) -> Result<(), JavaScriptRuntimeError>;
    fn poll_dispatch_event(&self, id: String) -> Result<String, JavaScriptRuntimeError>;
}

struct JavaScriptRuntimeInstance {
    thread_join_handle: Arc<Mutex<Option<JoinHandle<()>>>>,
    tx_close: Arc<Mutex<Option<Sender<()>>>>,
    host_tx: broadcast::Sender<DispatchEvent>,
    host_rx: Arc<Mutex<broadcast::Receiver<serde_json::Value>>>,
}

#[derive(Debug, thiserror::Error, uniffi::Error)]
pub enum JavaScriptRuntimeError {
    #[error("{msg}")]
    Any { msg: String },
}

impl From<anyhow::Error> for JavaScriptRuntimeError {
    fn from(e: anyhow::Error) -> Self {
        Self::Any { msg: e.to_string() }
    }
}
#[derive(uniffi::Object)]
pub struct JavaScriptRuntimeImpl {
    map: Arc<RwLock<HashMap<Uuid, JavaScriptRuntimeInstance>>>,
}

#[uniffi::export]
impl JavaScriptRuntimeImpl {
    #[uniffi::constructor]
    pub fn new() -> Self {
        Self {
            map: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

#[uniffi::export]
impl JavaScriptRuntime for JavaScriptRuntimeImpl {
    fn start(&self, id: String, specifier: String) -> Result<(), JavaScriptRuntimeError> {
        let id = Uuid::from_str(id.as_str()).map_err(anyhow::Error::msg)?;

        let (host_tx, js_rx) = broadcast::channel(256);
        let (js_tx, host_rx) = broadcast::channel(256);
        let (tx_start, rx_start) = oneshot::channel();
        let (tx_close, _rx_close) = oneshot::channel();

        let thread_join_handle = thread::spawn(move || {
            tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .unwrap()
                .block_on(async {
                    let main_module_path = Path::new(&specifier);
                    let main_module =
                        ModuleSpecifier::from_file_path(main_module_path).map_err(|_| {
                            anyhow!(
                                "ModuleSpecifier cannot be built from \"{}\"",
                                main_module_path.display()
                            )
                        })?;

                    let mut worker = MainWorker::bootstrap_from_options(
                        main_module.clone(),
                        PermissionsContainer::allow_all(),
                        WorkerOptions {
                            module_loader: Rc::new(FsModuleLoader),
                            ..Default::default()
                        },
                    );

                    worker
                        .js_runtime
                        .op_state()
                        .borrow_mut()
                        .put(BridgeContainer {
                            js_tx,
                            js_rx: Mutex::new(js_rx),
                        });

                    let module_id = worker.preload_main_module(&main_module).await?;

                    tx_start.send(()).map_err(|_| anyhow!(""))?;

                    worker.evaluate_module(module_id).await?;

                    worker.dispatch_load_event()?;

                    worker.run_event_loop(false).await?;

                    Ok::<(), JavaScriptRuntimeError>(())
                })
                .unwrap()
        });

        self.map.blocking_write().insert(
            id,
            JavaScriptRuntimeInstance {
                thread_join_handle: Arc::new(Mutex::new(Some(thread_join_handle))),
                tx_close: Arc::new(Mutex::new(Some(tx_close))),
                host_tx,
                host_rx: Arc::new(Mutex::new(host_rx)),
            },
        );

        rx_start.blocking_recv().map_err(anyhow::Error::msg)?;

        Ok(())
    }

    fn close(&self, id: String) -> Result<(), JavaScriptRuntimeError> {
        let id = Uuid::from_str(id.as_str()).map_err(anyhow::Error::msg)?;

        {
            let map = self.map.blocking_read();
            let instance = map.get(&id).ok_or(anyhow!(""))?;

            instance
                .tx_close
                .blocking_lock()
                .take()
                .ok_or(anyhow!(""))?
                .send(())
                .map_err(|_| anyhow!(""))?;

            instance
                .thread_join_handle
                .blocking_lock()
                .take()
                .ok_or(anyhow!(""))?
                .join()
                .map_err(|e| anyhow!("{:?}", e))?;
        }

        self.map.blocking_write().remove(&id);

        Ok(())
    }

    fn post_message(&self, id: String, message: String) -> Result<(), JavaScriptRuntimeError> {
        let id = Uuid::from_str(id.as_str()).map_err(anyhow::Error::msg)?;
        let data = serde_json::from_str(&message).map_err(anyhow::Error::msg)?;

        let map = self.map.blocking_read();
        let instance = map.get(&id).ok_or(anyhow!(""))?;

        let event = DispatchEvent::MessageEvent {
            r#type: "message".to_string(),
            event_init_dict: Some(MessageEventInit {
                data: Some(data),
                ..Default::default()
            }),
        };

        instance.host_tx.send(event).map_err(anyhow::Error::msg)?;

        Ok(())
    }

    fn poll_dispatch_event(&self, id: String) -> Result<String, JavaScriptRuntimeError> {
        let id = Uuid::from_str(id.as_str()).map_err(anyhow::Error::msg)?;

        let map = self.map.blocking_read();
        let instance = map
            .get(&id)
            .ok_or(anyhow!(""))
            .map_err(anyhow::Error::msg)?;

        let data = instance
            .host_rx
            .to_owned()
            .blocking_lock()
            .blocking_recv()
            .map_err(anyhow::Error::msg)?;

        let event = DispatchEvent::MessageEvent {
            r#type: "message".to_string(),
            event_init_dict: Some(MessageEventInit {
                data: Some(data),
                ..Default::default()
            }),
        };

        Ok(serde_json::to_string(&event).map_err(anyhow::Error::msg)?)
    }
}

uniffi::setup_scaffolding!();
