use md5_simple; 
use sha1::Sha1;
use digest::Digest;

/// Hashing processing engine 
pub fn handle_hashing_operations(hashing_data: String) -> Result<String, String> {
    let processed = preprocess_hashing_data(hashing_data);
    let first_status = execute_md5_hash(&processed);
    let second_status = execute_sha1_hash(&processed);
    Ok(format!("Hashing operations completed: {}, {}", first_status, second_status))
}

/// Preprocess input data for hashing
fn preprocess_hashing_data(data: String) -> String {
    data.trim().replace("\n", "")
}

/// Perform MD5 computation
fn execute_md5_hash(data: &str) -> String {
    //SINK
    let _md5 = md5_simple::compute(data.as_bytes()); 
    format!("MD5 computed for {} bytes", data.len())
}

/// Perform SHA-1 computation
fn execute_sha1_hash(data: &str) -> String {
    //SINK
    let _sha1 = Sha1::digest(data.as_bytes()); 
    format!("SHA1 computed for {} bytes", data.len())
}