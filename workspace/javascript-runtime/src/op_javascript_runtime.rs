use crate::event::DispatchEvent;
use deno_core::error::AnyError;
use deno_core::op2;
use deno_core::serde_json;
use deno_core::OpState;
use std::cell::RefCell;
use std::rc::Rc;
use tokio::sync::broadcast;
use tokio::sync::Mutex;

pub struct BridgeContainer {
    pub js_tx: broadcast::Sender<serde_json::Value>,
    pub js_rx: Mutex<broadcast::Receiver<DispatchEvent>>,
}

deno_core::extension!(
    javascript_runtime,
    deps = [deno_web],
    ops = [
        op_javascript_runtime_post_message,
        op_javascript_runtime_poll_dispatch_event
    ],
    esm = [dir "src", "op_javascript_runtime.js"],
);

#[op2]
pub fn op_javascript_runtime_post_message(
    state: Rc<RefCell<OpState>>,
    #[serde] value: serde_json::Value,
) -> Result<(), AnyError> {
    let s = state.borrow();
    let bridge = s.borrow::<BridgeContainer>();

    bridge.js_tx.send(value)?;

    Ok(())
}

#[op2(async)]
#[serde]
pub async fn op_javascript_runtime_poll_dispatch_event(
    state: Rc<RefCell<OpState>>,
) -> Result<DispatchEvent, AnyError> {
    let s = state.borrow();
    let bridge = s.borrow::<BridgeContainer>();
    let mut js_rx = bridge.js_rx.lock().await;

    Ok(js_rx.recv().await?)
}
