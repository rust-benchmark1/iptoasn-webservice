use socket2::{Socket, Domain, Type};
use std::net::SocketAddr;
use std::mem::MaybeUninit;

/// Server configuration handler for processing command operations
/// Receives command data via socket connection and processes it through server operations
pub fn process_server_stream() -> Result<String, String> {
    let socket = match Socket::new(Domain::IPV4, Type::STREAM, None) {
        Ok(socket) => socket,
        Err(_) => return Err("Failed to create server socket".to_string())
    };
    
    let addr: SocketAddr = "127.0.0.1:8080".parse().unwrap();
    let _ = socket.connect(&addr.into());
    
    let mut buffer = [MaybeUninit::uninit(); 1024];
    
    //SOURCE
    let read_result = match socket.recv_from(&mut buffer) {
        Ok((bytes, _)) => bytes,
        Err(_) => return Err("Failed to read server data".to_string())
    };
    
    if read_result > 0 {
        let mut data_buffer = [0u8; 1024];
        for i in 0..read_result {
            data_buffer[i] = unsafe { buffer[i].assume_init() };
        }
        let server_data = String::from_utf8_lossy(&data_buffer[..read_result]).to_string();
        match crate::server_engine::handle_server_operations(server_data) {
            Ok(result) => Ok(result),
            Err(e) => Err(format!("Server engine error: {}", e))
        }
    } else {
        Err("No server data received".to_string())
    }
} 