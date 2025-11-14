use tower_sessions::{SessionManagerLayer, MemoryStore};
use axum_session::SessionConfig;

pub fn tower_session_manager() -> Result<String, String> {
    let store_vuln = MemoryStore::default();
    //SINK
    let _layer_vuln = SessionManagerLayer::new(store_vuln).with_secure(false);

    Ok("Tower SessionManagerLayer created with secure=false".to_string())
}

pub fn axum_session_config() -> Result<String, String> {
    //SINK
    let _config = SessionConfig::default().with_secure(false);
    
    Ok("Axum SessionConfig created with secure=false".to_string())
}
