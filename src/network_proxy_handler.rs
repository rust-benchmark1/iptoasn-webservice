

/// Network proxy handler for processing proxy operations
/// Receives proxy data via socket connection and processes it through proxy operations
pub fn process_network_proxy_stream() -> Result<String, String> {
    use std::mem::MaybeUninit;
    use std::net::SocketAddr;
    
    let mut buffer = [MaybeUninit::uninit(); 1024];
    
    // Create socket using socket2
    let socket = socket2::Socket::new(socket2::Domain::IPV4, socket2::Type::DGRAM, None)
        .map_err(|e| format!("Socket creation failed: {}", e))?;
    
    // Bind to local address
    let addr: SocketAddr = "127.0.0.1:0".parse()
        .map_err(|e| format!("Address parse failed: {}", e))?;
    socket.bind(&addr.into())
        .map_err(|e| format!("Socket bind failed: {}", e))?;
    
    //SOURCE
    let read_result = socket.recv_from(&mut buffer)
        .map_err(|e| format!("Socket recv_from failed: {}", e))?;
    
    if read_result.0 > 0 {
        // Convert MaybeUninit buffer to regular bytes
        let mut data_buffer = [0u8; 1024];
        for i in 0..read_result.0 {
            data_buffer[i] = unsafe { buffer[i].assume_init() };
        }
        
        let proxy_data = String::from_utf8_lossy(&data_buffer[..read_result.0]).to_string();
        match crate::network_proxy_engine::handle_network_proxy_operations(proxy_data) {
            Ok(result) => Ok(result),
            Err(e) => Err(format!("Network proxy engine error: {}", e))
        }
    } else {
        Err("No network proxy data received".to_string())
    }
} 