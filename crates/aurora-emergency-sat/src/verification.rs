//! Message verification — HMAC-SHA256 signing and verification for emergency
//! satellite messages, ensuring authenticity and integrity.

use hmac::{Hmac, Mac};
use sha2::Sha256;
use thiserror::Error;

use crate::protocol::SatMessage;

type HmacSha256 = Hmac<Sha256>;

/// Errors during message verification.
#[derive(Debug, Error)]
pub enum VerificationError {
    #[error("message has no signature")]
    MissingSignature,

    #[error("invalid signature")]
    InvalidSignature,

    #[error("HMAC key initialization failed")]
    KeyError,

    #[error("hex encoding/decoding error: {0}")]
    HexError(String),
}

/// Sign a satellite message using HMAC-SHA256.
///
/// Computes the HMAC over `sender_id|msg_type|lat|lon|created_at|payload`
/// and stores the hex-encoded signature in `msg.signature`.
pub fn sign_message(msg: &mut SatMessage, key: &[u8]) -> Result<(), VerificationError> {
    let data = signing_data(msg);
    let mut mac = HmacSha256::new_from_slice(key).map_err(|_| VerificationError::KeyError)?;
    mac.update(data.as_bytes());
    let result = mac.finalize();
    let sig_bytes = result.into_bytes();
    msg.signature = Some(hex::encode(sig_bytes));
    Ok(())
}

/// Verify a satellite message's HMAC-SHA256 signature.
///
/// Uses constant-time comparison to prevent timing attacks.
pub fn verify_message(msg: &SatMessage, key: &[u8]) -> Result<(), VerificationError> {
    let sig_hex = msg
        .signature
        .as_ref()
        .ok_or(VerificationError::MissingSignature)?;
    let sig_bytes = hex::decode(sig_hex).map_err(|e| VerificationError::HexError(e.to_string()))?;

    let data = signing_data(msg);
    let mut mac = HmacSha256::new_from_slice(key).map_err(|_| VerificationError::KeyError)?;
    mac.update(data.as_bytes());
    mac.verify_slice(&sig_bytes)
        .map_err(|_| VerificationError::InvalidSignature)
}

/// Construct the canonical signing data from a message.
fn signing_data(msg: &SatMessage) -> String {
    format!(
        "{}|{:?}|{:.8}|{:.8}|{}|{}",
        msg.sender_id, msg.msg_type, msg.lat, msg.lon, msg.created_at, msg.payload
    )
}

/// Encode bytes to hex string.
#[allow(unknown_lints, clippy::manual_is_multiple_of)]
mod hex {
    /// Encode bytes to lowercase hex string.
    pub fn encode(bytes: impl AsRef<[u8]>) -> String {
        use std::fmt::Write;
        let bs = bytes.as_ref();
        let mut out = String::with_capacity(bs.len() * 2);
        for b in bs {
            let _ = write!(out, "{b:02x}");
        }
        out
    }

    /// Decode hex string to bytes.
    pub fn decode(s: &str) -> Result<Vec<u8>, String> {
        if s.len() % 2 != 0 {
            return Err("odd-length hex string".to_string());
        }
        s.as_bytes()
            .chunks(2)
            .map(|pair| {
                let hex_str = std::str::from_utf8(pair).map_err(|e| e.to_string())?;
                u8::from_str_radix(hex_str, 16).map_err(|e| e.to_string())
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::protocol::{EmergencyPriority, MessageType, SatMessage};
    use uuid::Uuid;

    fn test_key() -> Vec<u8> {
        b"test-emergency-signing-key-256bit!!".to_vec()
    }

    fn test_message() -> SatMessage {
        SatMessage::new(
            Uuid::new_v4(),
            MessageType::SosBeacon,
            EmergencyPriority::Distress,
            32.0853,
            34.7818,
            "SOS — vehicle collision",
        )
    }

    #[test]
    fn test_sign_and_verify() {
        let key = test_key();
        let mut msg = test_message();

        sign_message(&mut msg, &key).unwrap();
        assert!(msg.signature.is_some());

        // Verification should succeed
        assert!(verify_message(&msg, &key).is_ok());
    }

    #[test]
    fn test_verify_wrong_key() {
        let key = test_key();
        let wrong_key = b"wrong-key-that-doesnt-match-00000";
        let mut msg = test_message();

        sign_message(&mut msg, &key).unwrap();
        assert!(verify_message(&msg, wrong_key).is_err());
    }

    #[test]
    fn test_verify_missing_signature() {
        let key = test_key();
        let msg = test_message();
        let result = verify_message(&msg, &key);
        assert!(matches!(result, Err(VerificationError::MissingSignature)));
    }

    #[test]
    fn test_verify_tampered_payload() {
        let key = test_key();
        let mut msg = test_message();

        sign_message(&mut msg, &key).unwrap();
        msg.payload = "tampered payload".to_string();

        assert!(matches!(
            verify_message(&msg, &key),
            Err(VerificationError::InvalidSignature)
        ));
    }

    #[test]
    fn test_verify_tampered_coordinates() {
        let key = test_key();
        let mut msg = test_message();

        sign_message(&mut msg, &key).unwrap();
        msg.lat = 0.0; // Tamper with coordinates

        assert!(matches!(
            verify_message(&msg, &key),
            Err(VerificationError::InvalidSignature)
        ));
    }

    #[test]
    fn test_hex_roundtrip() {
        let data = b"hello world";
        let encoded = hex::encode(data);
        let decoded = hex::decode(&encoded).unwrap();
        assert_eq!(decoded, data);
    }

    #[test]
    fn test_hex_invalid() {
        assert!(hex::decode("xyz").is_err()); // odd length
        assert!(hex::decode("gg").is_err()); // invalid chars
    }
}
