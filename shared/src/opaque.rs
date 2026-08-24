use serde::{Deserialize, Serialize};

// opaque-ke message bytes only; shared_lib doesn't depend on opaque-ke directly.

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct RegisterStartRequest {
    pub account_id: String,
    pub opaque_message: Vec<u8>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct RegisterStartResponse {
    pub opaque_message: Vec<u8>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct RegisterFinishRequest {
    pub account_id: String,
    pub opaque_message: Vec<u8>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct RegisterFinishResponse {
    pub success: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct LoginStartRequest {
    pub account_id: String,
    pub device_id: String,
    pub opaque_message: Vec<u8>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct LoginStartResponse {
    pub opaque_message: Vec<u8>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct LoginFinishRequest {
    pub account_id: String,
    pub device_id: String,
    pub opaque_message: Vec<u8>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct LoginFinishResponse {
    pub session_token: String,
}
