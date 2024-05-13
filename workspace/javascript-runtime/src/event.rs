use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct EventInit {
    pub bubbles: Option<bool>,
    pub cancelable: Option<bool>,
    pub composed: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct ErrorEventInit {
    pub bubbles: Option<bool>,
    pub cancelable: Option<bool>,
    pub composed: Option<bool>,

    pub colno: Option<f64>,
    pub error: Option<serde_json::Value>,
    pub filename: Option<String>,
    pub lineno: Option<f64>,
    pub message: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct CloseEventInit {
    pub bubbles: Option<bool>,
    pub cancelable: Option<bool>,
    pub composed: Option<bool>,

    pub code: Option<f64>,
    pub reason: Option<String>,
    pub was_clean: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct MessageEventInit {
    pub bubbles: Option<bool>,
    pub cancelable: Option<bool>,
    pub composed: Option<bool>,

    pub data: Option<serde_json::Value>,
    pub last_event_id: Option<String>,
    pub origin: Option<String>,
    pub ports: Option<()>,
    pub source: Option<()>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct CustomEventInit {
    pub bubbles: Option<bool>,
    pub cancelable: Option<bool>,
    pub composed: Option<bool>,

    pub detail: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "constructor")]
#[serde(rename_all = "camelCase")]
pub enum DispatchEvent {
    Event {
        r#type: String,
        event_init_dict: Option<EventInit>,
    },
    ErrorEvent {
        r#type: String,
        event_init_dict: Option<ErrorEventInit>,
    },
    CloseEvent {
        r#type: String,
        event_init_dict: Option<CloseEventInit>,
    },
    MessageEvent {
        r#type: String,
        event_init_dict: Option<MessageEventInit>,
    },
    CustomEvent {
        r#type: String,
        event_init_dict: Option<CustomEventInit>,
    },
}
