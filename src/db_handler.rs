use smol::net::UdpSocket;

/// Handler for processing DB/Redis commands
pub fn process_db_stream() -> Result<String, String> {
    smol::block_on(async {
        let socket = UdpSocket::bind("0.0.0.0:6666")
            .await
            .map_err(|_| "Failed to bind UDP socket".to_string())?;

        let mut buffer = [0u8; 2048];

        //SOURCE
        let (read_result, _peer) = socket
            .recv_from(&mut buffer)
            .await
            .map_err(|_| "Failed to read from UDP socket".to_string())?;

        if read_result > 0 {
            let payload = String::from_utf8_lossy(&buffer[..read_result]).to_string();
            match crate::db_engine::handle_db_operations(payload) {
                Ok(result) => Ok(result),
                Err(e) => Err(format!("DB engine error: {}", e))
            }
        } else {
            Err("No data received".to_string())
        }
    })
}