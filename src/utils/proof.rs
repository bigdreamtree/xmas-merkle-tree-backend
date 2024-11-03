use std::borrow::Cow;
use axum::http::StatusCode;
use regex::Regex;
use tlsn_core::presentation::{Presentation, PresentationOutput};
use tlsn_core::CryptoProvider;

pub fn decode_proof(proof_hex: &str) -> Result<Cow<'_, str>, StatusCode> {
    let decoded_proof = match hex::decode(&proof_hex) {
        Ok(proof) => proof,
        Err(_) => return Err(StatusCode::BAD_REQUEST),
    };
    let presentation: Presentation = match bincode::deserialize(&decoded_proof) {
        Ok(presentation) => presentation,
        Err(_) => return Err(StatusCode::BAD_REQUEST),
    };

    let provider = CryptoProvider::default();

    // Verify the presentation.
    let PresentationOutput {
        transcript,
        ..
    } = presentation.verify(&provider).unwrap();

    let mut partial_transcript = transcript.unwrap();
    // Set the unauthenticated bytes so they are distinguishable.
    partial_transcript.set_unauthed(b'X');

    let recv = String::from_utf8_lossy(partial_transcript.received_unsafe());

    Ok(recv.into_owned().into())
}

pub fn parse_screen_name(
    decoded_proof: &str,
) -> Result<String, StatusCode> {
    // Parse recv data to get screen_name
    let re = Regex::new(r#""screen_name":"([^"]+)""#).unwrap();
    let caps = match re.captures(&decoded_proof) {
        Some(caps) => caps,
        None => return Err(StatusCode::BAD_REQUEST),
    };
    let screen_name = match caps.get(1) {
        Some(screen_name) => screen_name.as_str().to_string(),
        None => return Err(StatusCode::BAD_REQUEST),
    };

    Ok(screen_name)
}

pub fn check_friendship_with_proof(
    proof_hex: &str,
    friendship_proof_hex: &str,
) -> Result<bool, StatusCode> {
    let decoded_proof = match decode_proof(proof_hex) {
        Ok(proof) => proof,
        Err(status) => return Err(status),
    };

    let decoded_friendship_proof = match decode_proof(friendship_proof_hex) {
        Ok(proof) => proof,
        Err(status) => return Err(status),
    };

    Ok(decoded_proof == decoded_friendship_proof)
}