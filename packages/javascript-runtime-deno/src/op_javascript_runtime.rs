use crate::event::DispatchEvent;
use deno_core::{error::AnyError, op2, serde_json, OpState};
use std::{cell::RefCell, rc::Rc, sync::Arc};
use tokio::sync::{
    broadcast::{Receiver, Sender},
    Mutex,
};

pub trait BridgeContainer: Clone {
    fn send(&self, value: serde_json::Value) -> Result<(), AnyError>;
    async fn recv(&self) -> Result<DispatchEvent, AnyError>;
}

#[derive(Clone)]
pub struct BridgeContainerImpl {
    tx: Sender<serde_json::Value>,
    rx: Arc<Mutex<Receiver<DispatchEvent>>>,
}

impl BridgeContainerImpl {
    pub fn from_broadcast_channel(
        tx: Sender<serde_json::Value>,
        rx: Receiver<DispatchEvent>,
    ) -> Self {
        BridgeContainerImpl {
            tx,
            rx: Arc::new(Mutex::new(rx)),
        }
    }
}

impl BridgeContainer for BridgeContainerImpl {
    fn send(&self, value: serde_json::Value) -> Result<(), AnyError> {
        self.tx.send(value)?;

        Ok(())
    }

    async fn recv(&self) -> Result<DispatchEvent, AnyError> {
        let event = self.rx.lock().await.recv().await?;

        Ok(event)
    }
}

deno_core::extension!(javascript_runtime,
    deps = [deno_web],
    parameters = [BC: BridgeContainer],
    ops = [
        op_javascript_runtime_post_message<BC>,
        op_javascript_runtime_poll_dispatch_event<BC>
    ],
    esm_entry_point = "ext:javascript_runtime/op_javascript_runtime.js",
    esm = [dir "src", "op_javascript_runtime.js"],
    options = {
        bc: BC,
    },
    state = |state, options| {
        state.put(options.bc);
    },
);

#[op2]
pub fn op_javascript_runtime_post_message<BC>(
    state: Rc<RefCell<OpState>>,
    #[serde] value: serde_json::Value,
) -> Result<(), AnyError>
where
    BC: BridgeContainer + 'static,
{
    println!("op_javascript_runtime_post_message: {}", value);

    let bc = state.borrow().borrow::<BC>().clone();

    bc.send(value)
}

#[op2(async)]
#[serde]
pub async fn op_javascript_runtime_poll_dispatch_event<BC>(
    state: Rc<RefCell<OpState>>,
    // #[smi] rid: ResourceId,
) -> Result<DispatchEvent, AnyError>
where
    BC: BridgeContainer + 'static,
{
    println!("op_javascript_runtime_poll_dispatch_event");

    let bc = state.borrow().borrow::<BC>().clone();

    let event = bc.recv().await?;

    println!(
        "op_javascript_runtime_poll_dispatch_event: {:?} {}",
        event,
        serde_json::to_string(&event)?
    );

    Ok(event)
}
