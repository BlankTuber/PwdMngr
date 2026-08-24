use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct EncryptedEvent {
    pub event_id: String,       // client UUID, stable across retries (dedup key)
    pub server_id: Option<i64>, // None until synced; also the sync cursor
    pub account_id: String,
    pub device_id: String,
    pub is_snapshot: bool, // true = full account state, supersedes earlier events
    pub encrypted_payload: Vec<u8>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct SyncPullRequest {
    #[serde(default)]
    pub after_server_id: Option<i64>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct SyncPullResponse {
    pub events: Vec<EncryptedEvent>,
    pub latest_server_id: Option<i64>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct SyncPushRequest {
    pub new_events: Vec<EncryptedEvent>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct SyncPushResponse {
    pub accepted_event_ids: Vec<String>,
    pub latest_server_id: i64,
}
