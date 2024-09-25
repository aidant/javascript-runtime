use anyhow::anyhow;
use deno_runtime::{
    deno_core::{FsModuleLoader, ModuleSpecifier},
    deno_permissions::PermissionsContainer,
    worker::{MainWorker, WorkerOptions},
};
use event::{DispatchEvent, MessageEventInit};
use op_javascript_runtime::BridgeContainerImpl;
use std::{collections::HashMap, path::Path, rc::Rc, str::FromStr, sync::Arc, thread::JoinHandle};
use tokio::sync::{broadcast, oneshot, Mutex, RwLock};
use uuid::Uuid;

mod event;
mod op_javascript_runtime;
mod stdio_to_logcat;

pub trait JavaScriptRuntime: Send + Sync {
    fn start(
        &self,
        root: String,
        id: String,
        specifier: String,
    ) -> Result<(), JavaScriptRuntimeError>;
    fn close(&self, id: String) -> Result<(), JavaScriptRuntimeError>;
    fn post_message(&self, id: String, message: String) -> Result<(), JavaScriptRuntimeError>;
    fn poll_dispatch_event(&self, id: String) -> Result<String, JavaScriptRuntimeError>;
}

struct JavaScriptRuntimeInstance {
    thread_join_handle: Arc<Mutex<Option<JoinHandle<Result<(), anyhow::Error>>>>>,
    tx_close: Arc<Mutex<Option<oneshot::Sender<()>>>>,
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
    fn start(
        &self,
        root: String,
        id: String,
        specifier: String,
    ) -> Result<(), JavaScriptRuntimeError> {
        stdio_to_logcat!();

        let id = Uuid::from_str(id.as_str()).map_err(anyhow::Error::msg)?;

        let (host_tx, js_rx) = broadcast::channel(256);
        let (js_tx, host_rx) = broadcast::channel(256);
        let (tx_start, rx_start) = oneshot::channel();
        let (tx_close, rx_close) = oneshot::channel();

        let thread_join_handle = Builder::new()
            .name("javascript_runtime".into())
            .spawn(move || {
                tokio::runtime::Builder::new_current_thread()
                    .enable_all()
                    .build()
                    .map_err(anyhow::Error::msg)?
                    .block_on(async move {
                        println!("start of runtime execution");

                        let main_module_path = Path::new(&root).join(&specifier);
                        let main_module = ModuleSpecifier::from_file_path(
                            main_module_path.as_path(),
                        )
                        .map_err(|_| {
                            anyhow!(
                                "ModuleSpecifier cannot be built from \"{}\"",
                                main_module_path.display()
                            )
                        })?;

                        let mut worker = MainWorker::bootstrap_from_options(
                            main_module.clone(),
                            PermissionsContainer::allow_all(),
                            WorkerOptions {
                                extensions: vec![
                                    op_javascript_runtime::javascript_runtime::init_ops_and_esm(
                                        BridgeContainerImpl::from_broadcast_channel(js_tx, js_rx),
                                    ),
                                ],
                                module_loader: Rc::new(FsModuleLoader),
                                ..Default::default()
                            },
                        );

                        let module_id = worker.preload_main_module(&main_module).await?;

                        tx_start
                            .send(())
                            .map_err(|_| anyhow!(r#"The thread "{id}" could not be started"#))?;

                        worker.evaluate_module(module_id).await?;

                        worker.dispatch_load_event()?;

                        tokio::select! {
                            res1 = worker.run_event_loop(false) => res1,
                            res2 = rx_close => res2.map_err(anyhow::Error::msg),
                        }?;

                        println!("end of runtime execution");

                        Ok::<(), anyhow::Error>(())
                    })
                    .map_err(anyhow::Error::msg)
            })
            .map_err(anyhow::Error::msg)?;

        match rx_start.blocking_recv() {
            Ok(_) => {
                self.map.blocking_write().insert(
                    id,
                    JavaScriptRuntimeInstance {
                        thread_join_handle: Arc::new(Mutex::new(Some(thread_join_handle))),
                        tx_close: Arc::new(Mutex::new(Some(tx_close))),
                        host_tx,
                        host_rx: Arc::new(Mutex::new(host_rx)),
                    },
                );
            }
            Err(e1) => thread_join_handle.join().map_err(|e2| {
                anyhow!(
                    r#"The thread "{id}" failed with "{e1}" and could not be joined due to "{:?}""#,
                    e2
                )
            })??,
        }

        Ok(())
    }

    fn close(&self, id: String) -> Result<(), JavaScriptRuntimeError> {
        let id = Uuid::from_str(id.as_str()).map_err(anyhow::Error::msg)?;

        {
            let map = self.map.blocking_read();
            let instance = map.get(&id).ok_or(anyhow!(
                r#"The thread "{id}" could not be found or has already been closed"#
            ))?;

            /*
                If the thread has already finished executing the event loop, the
                rx_close side of the oneshot channel will he dropped. So most if
                we are unable to send a message to the channel, it means the
                thread has already finished executing.
            */
            let _ = instance
                .tx_close
                .blocking_lock()
                .take()
                .ok_or(anyhow!(r#"The thread "{id}" has already been closed"#))?
                .send(());

            instance
                .thread_join_handle
                .blocking_lock()
                .take()
                .ok_or(anyhow!(r#"The thread "{id}" has already been closed"#))?
                .join()
                .map_err(|e| {
                    anyhow!(r#"The thread "{id}" could not be joined due to "{:?}""#, e)
                })??;
        }

        self.map.blocking_write().remove(&id);

        Ok(())
    }

    fn post_message(&self, id: String, message: String) -> Result<(), JavaScriptRuntimeError> {
        let id = Uuid::from_str(id.as_str()).map_err(anyhow::Error::msg)?;
        let data = serde_json::from_str(&message).map_err(|e| {
            anyhow!(r#"Unable to deserialize json message for thread "{id}" failed with "{e}""#)
        })?;

        let map = self.map.blocking_read();
        let instance = map.get(&id).ok_or(anyhow!(
            r#"The thread "{id}" could not be found or has already been closed"#
        ))?;

        let event = DispatchEvent::MessageEvent {
            r#type: "message".to_string(),
            event_init_dict: Some(MessageEventInit {
                data: Some(data),
                ..Default::default()
            }),
        };

        instance.host_tx.send(event.clone()).map_err(|e| {
            anyhow!(
                r#"The host_tx channel for thread "{id}" failed to send event "{:?}" with "{e}""#,
                event
            )
        })?;

        Ok(())
    }

    fn poll_dispatch_event(&self, id: String) -> Result<String, JavaScriptRuntimeError> {
        let id = Uuid::from_str(id.as_str()).map_err(anyhow::Error::msg)?;

        let map = self.map.blocking_read();
        let instance = map.get(&id).ok_or(anyhow!(
            r#"The thread "{id}" could not be found or has already been closed"#
        ))?;

        let data = instance
            .host_rx
            .to_owned()
            .blocking_lock()
            .blocking_recv()
            .map_err(|e| anyhow!(r#"The host_rx channel for thread "{id}" failed with "{e}""#))?;

        let event = DispatchEvent::MessageEvent {
            r#type: "message".to_string(),
            event_init_dict: Some(MessageEventInit {
                data: Some(data),
                ..Default::default()
            }),
        };

        Ok(serde_json::to_string(&event).map_err(|e| {
            anyhow!(r#"Unable to serialize event to json for thread "{id}" failed with "{e}""#)
        })?)
    }
}

uniffi::setup_scaffolding!();
