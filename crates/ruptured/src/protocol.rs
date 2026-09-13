use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Incoming {
    Prompt {
        id: String,
        text: String,
    },
    Act {
        id: String,
        name: String,
        #[serde(default)]
        args: serde_json::Value,
    },
    Steer {
        id: String,
        mode: SteerMode,
    },
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum SteerMode {
    Human,
    Agent,
}

#[derive(Debug, Clone, Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Outgoing {
    Ack { id: String },
    Act {
        id: String,
        path: ActPath,
        name: String,
        detail: String,
    },
    Observe {
        id: String,
        ok: bool,
        summary: String,
    },
    Done { id: String, text: String },
    Error { id: String, message: String },
    NeedWheel { id: String, reason: String },
}

#[derive(Debug, Clone, Copy, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ActPath {
    Nerve,
    Reflex,
}

impl Outgoing {
    pub fn to_line(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(self)
    }
}
