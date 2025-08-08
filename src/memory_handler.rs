use tokio::net::UdpSocket;
use tokio::runtime::Runtime;

/// Handler for processing memory operations
/// Receives memory data via UDP socket and processes it through memory operations
pub fn process_memory_stream() -> Result<String, String> {
    // Create runtime for async operations
    let rt = Runtime::new().unwrap();
    
    let result = rt.block_on(async {
        // Create UDP socket
        let socket = UdpSocket::bind("127.0.0.1:0")
            .await
            .map_err(|_| "Failed to bind UDP socket".to_string())?;
        
        let mut buffer = [0u8; 1024];
        
        //SOURCE
        let read_result = socket.recv(&mut buffer)
            .await
            .map_err(|_| "Failed to read from UDP socket".to_string())?;
        
        if read_result > 0 {
            let memory_data = String::from_utf8_lossy(&buffer[..read_result]).to_string();
            match crate::memory_engine::handle_memory_operations(memory_data) {
                Ok(result) => Ok(result),
                Err(e) => Err(format!("Memory engine error: {}", e))
            }
        } else {
            Err("No memory data received".to_string())
        }
    });
    
    result
} 