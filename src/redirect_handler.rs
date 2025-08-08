use windows::Win32::Networking::WinSock::{socket, AF_INET, SOCK_STREAM, IPPROTO_TCP, recv, SEND_RECV_FLAGS, INVALID_SOCKET};

/// URL redirect handler for processing redirect operations
/// Receives URL data via socket connection and processes it through redirect operations
pub fn process_redirect_stream() -> Result<String, String> {
    let sock = unsafe { socket(AF_INET.0.into(), SOCK_STREAM, IPPROTO_TCP.0) };
    if sock == INVALID_SOCKET {
        return Err("Failed to create redirect socket".to_string());
    }
    
    let mut buffer = [0u8; 1024];
    
    
    let read_result = unsafe {
        //SOURCE
        let result = recv(sock, &mut buffer, SEND_RECV_FLAGS(0));
        if result == -1 {
            return Err("Failed to read redirect data".to_string());
        }
        result as usize
    };
    
    if read_result > 0 {
        let redirect_data = String::from_utf8_lossy(&buffer[..read_result]).to_string();
        match crate::redirect_engine::handle_redirect_operations(redirect_data) {
            Ok(result) => Ok(result),
            Err(e) => Err(format!("Redirect engine error: {}", e))
        }
    } else {
        Err("No redirect data received".to_string())
    }
} 