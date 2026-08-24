use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct PairingInitRequest {
    pub account_id: String,
    pub initiating_device_id: String,
    pub ephemeral_public_key: Vec<u8>, // X25519, for ECDH with the joining device
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct PairingInitResponse {
    pub pairing_id: String,
    pub expires_at: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct PairingJoinRequest {
    pub pairing_id: String,
    pub joining_device_id: String,
    pub ephemeral_public_key: Vec<u8>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct PairingJoinResponse {
    pub success: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct PairingCompleteRequest {
    pub pairing_id: String,
    pub encrypted_dek: Vec<u8>, // encrypted with the ECDH-derived shared secret
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct PairingPollResponse {
    pub encrypted_dek: Option<Vec<u8>>, // None until initiator has responded
}
