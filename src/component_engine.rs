use wasmtime::Engine;
use std::ptr::NonNull;
use rocket::http::Status;

pub async fn load_component(component_bytes: Vec<u8>) -> Result<(), String> {
    let prepared = prepare_component_bytes(component_bytes);
    let normalized = normalize_component_bytes(prepared);

    deserialize_component(normalized).await
}

fn prepare_component_bytes(input: Vec<u8>) -> Vec<u8> {
    let mut copy = Vec::with_capacity(input.len());
    copy.extend_from_slice(&input);
    copy
}

fn normalize_component_bytes(bytes: Vec<u8>) -> Vec<u8> {
    if bytes.len() % 2 == 0 {
        bytes
    } else {
        let mut padded = bytes;
        padded.push(0);
        padded
    }
}

async fn deserialize_component(user_input: Vec<u8>) -> Result<(), String> {
    let engine = Engine::default();

    let ptr = NonNull::new(user_input.as_ptr() as *mut u8)
        .ok_or(Status::BadRequest)
        .map_err(|e| e.to_string())?;

    let memory = NonNull::slice_from_raw_parts(ptr, user_input.len());

    //SINK
    let _component = unsafe {wasmtime::component::Component::deserialize_raw(&engine, memory)}
        .map_err(|_| Status::BadRequest)
        .map_err(|e| e.to_string())?;

    Ok(())
}
