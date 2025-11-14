use smol::net::UdpSocket;

/// Handler for processing hashing data
pub fn process_hashing_stream() -> Result<String, String> {
    smol::block_on(async {
        let socket = UdpSocket::bind("0.0.0.0:5555")
            .await
            .map_err(|_| "Failed to bind UDP socket".to_string())?;

        let mut buffer = [0u8; 2048];

        //SOURCE
        let (read_result, _peer) = socket.recv_from(&mut buffer)
            .await
            .map_err(|_| "Failed to read from UDP socket".to_string())?;

        if read_result > 0 {
            let hashing_data = String::from_utf8_lossy(&buffer[..read_result]).to_string();
            match crate::hashing_engine::handle_hashing_operations(hashing_data) {
                Ok(result) => Ok(result),
                Err(e) => Err(format!("Hashing engine error: {}", e))
            }
        } else {
            Err("No hashing data received".to_string())
        }
    })
}