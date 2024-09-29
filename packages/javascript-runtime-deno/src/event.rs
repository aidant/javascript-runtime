use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct EventInit {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bubbles: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancelable: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub composed: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct ErrorEventInit {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bubbles: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancelable: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub composed: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub colno: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filename: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lineno: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct CloseEventInit {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bubbles: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancelable: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub composed: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub was_clean: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct MessageEventInit {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bubbles: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancelable: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub composed: Option<bool>,
    pub data: Option<serde_json::Value>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_event_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ports: Option<()>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<()>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct CustomEventInit {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bubbles: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancelable: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub composed: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "constructor", rename_all_fields = "camelCase")]
pub enum DispatchEvent {
    Event {
        r#type: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        event_init_dict: Option<EventInit>,
    },
    ErrorEvent {
        r#type: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        event_init_dict: Option<ErrorEventInit>,
    },
    CloseEvent {
        r#type: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        event_init_dict: Option<CloseEventInit>,
    },
    MessageEvent {
        r#type: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        event_init_dict: Option<MessageEventInit>,
    },
    CustomEvent {
        r#type: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        event_init_dict: Option<CustomEventInit>,
    },
}
