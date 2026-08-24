use crate::*;

fn roundtrip<
    T: serde::Serialize + for<'de> serde::Deserialize<'de> + PartialEq + std::fmt::Debug,
>(
    v: &T,
) {
    let bytes = rmp_serde::to_vec(v).unwrap();
    let decoded: T = rmp_serde::from_slice(&bytes).unwrap();
    assert_eq!(*v, decoded);
}

#[test]
fn encrypted_event_unsynced() {
    roundtrip(&EncryptedEvent {
        event_id: "evt_01HZY".into(),
        server_id: None,
        account_id: "acct_123".into(),
        device_id: "dev_abc".into(),
        is_snapshot: false,
        encrypted_payload: vec![0xDE, 0xAD, 0xBE, 0xEF],
    });
}

#[test]
fn encrypted_event_synced_snapshot() {
    roundtrip(&EncryptedEvent {
        event_id: "evt_01HZZ".into(),
        server_id: Some(49),
        account_id: "acct_123".into(),
        device_id: "dev_def".into(),
        is_snapshot: true,
        encrypted_payload: vec![1, 2, 3],
    });
}

#[test]
fn encrypted_event_empty_payload() {
    roundtrip(&EncryptedEvent {
        event_id: "evt_empty".into(),
        server_id: Some(1),
        account_id: "acct_123".into(),
        device_id: "dev_abc".into(),
        is_snapshot: false,
        encrypted_payload: vec![],
    });
}

#[test]
fn sync_pull_request_defaults_when_absent() {
    let decoded: SyncPullRequest = rmp_serde::from_slice(
        &rmp_serde::to_vec(&std::collections::BTreeMap::<String, i64>::new()).unwrap(),
    )
    .unwrap();
    assert_eq!(decoded.after_server_id, None);
}

#[test]
fn sync_pull_response_empty_events() {
    roundtrip(&SyncPullResponse {
        events: vec![],
        latest_server_id: None,
    });
}

#[test]
fn sync_push_response_empty_accepted() {
    roundtrip(&SyncPushResponse {
        accepted_event_ids: vec![],
        latest_server_id: 0,
    });
}

#[test]
fn opaque_messages_roundtrip() {
    roundtrip(&RegisterStartRequest {
        account_id: "acct_123".into(),
        opaque_message: vec![1, 2, 3],
    });
    roundtrip(&LoginFinishResponse {
        session_token: "tok_abc123".into(),
    });
}

#[test]
fn pairing_messages_roundtrip() {
    roundtrip(&PairingInitResponse {
        pairing_id: "pair_xyz".into(),
        expires_at: 1_700_000_000,
    });
    roundtrip(&PairingPollResponse {
        encrypted_dek: None,
    });
    roundtrip(&PairingPollResponse {
        encrypted_dek: Some(vec![9, 9, 9]),
    });
}

#[test]
fn sync_error_variants_roundtrip() {
    roundtrip(&SyncError::Unauthorized);
    roundtrip(&SyncError::InvalidEvent {
        event_id: "evt_bad".into(),
        reason: "malformed payload".into(),
    });
}
