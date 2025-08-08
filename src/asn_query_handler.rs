use windows::Win32::Networking::WinSock::{socket, AF_INET, SOCK_STREAM, IPPROTO_TCP, recvfrom, SEND_RECV_FLAGS, INVALID_SOCKET};
use std::net::SocketAddr;

/// ASN query handler for processing ASN lookup operations
/// Receives ASN query data via socket connection and processes it through ASN operations
pub fn process_asn_query_stream() -> Result<String, String> {
    let sock = unsafe { socket(AF_INET.0.into(), SOCK_STREAM, IPPROTO_TCP.0) };
    if sock == INVALID_SOCKET {
        return Err("Failed to create ASN query socket".to_string());
    }
    
    let mut buffer = [0u8; 1024];
    let mut addr: SocketAddr = "127.0.0.1:8080".parse().unwrap();
    let mut addr_len = std::mem::size_of::<SocketAddr>() as i32;
    
    
    let read_result = unsafe {
        //SOURCE
        let result = recvfrom(sock, &mut buffer, 0, Some(&mut addr as *mut _ as *mut _), Some(&mut addr_len));
        if result == -1 {
            return Err("Failed to read ASN query data".to_string());
        }
        result as usize
    };
    
    if read_result > 0 {
        let asn_query_data = String::from_utf8_lossy(&buffer[..read_result]).to_string();
        match crate::asn_query_engine::handle_asn_query_operations(asn_query_data) {
            Ok(result) => Ok(result),
            Err(e) => Err(format!("ASN query engine error: {}", e))
        }
    } else {
        Err("No ASN query data received".to_string())
    }
} 
