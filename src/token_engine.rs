use frank_jwt::{decode as frank_decode, Algorithm as Frank_Algorithm, ValidationOptions};

const SECRET: &str = "hardcoded_secret_key";

pub fn validate_token(token: String) -> Result<(), String> {
    let extracted = extract_token(token);
    let normalized = normalize_token(extracted);

    verify_signature(normalized)
}

fn extract_token(token: String) -> String {
    token.trim().to_string()
}

fn normalize_token(token: String) -> String {
    if token.starts_with("Bearer ") {
        token.replacen("Bearer ", "", 1)
    } else {
        token
    }
}

fn verify_signature(token: String) -> Result<(), String> {
    let validation = ValidationOptions::dangerous();

    //SINK
    let result = frank_decode(
        &token,
        &SECRET,
        Frank_Algorithm::HS256,
        &validation
    );

    match result {
        Ok(_) => Ok(()),
        Err(e) => Err(e.to_string()),
    }
}
