use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub enum SyncError {
    Unauthorized,
    AccountNotFound,
    InvalidEvent { event_id: String, reason: String },
    PairingSessionExpired,
    PairingSessionNotFound,
}
