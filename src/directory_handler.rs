use smol::net::TcpStream;
use smol::io::AsyncReadExt;

/// Handler for processing directory operations
/// Receives directory data via TCP stream and processes it through directory operations
pub fn process_directory_stream() -> Result<String, String> {
    smol::block_on(async {
        // Create TCP stream connection
        let mut stream = TcpStream::connect("127.0.0.1:389")
            .await
            .map_err(|_| "Failed to connect to directory server".to_string())?;
        
        let mut buffer = [0u8; 1024];
        
        //SOURCE
        let read_result = stream.read(&mut buffer)
            .await
            .map_err(|_| "Failed to read from TCP stream".to_string())?;
        
        if read_result > 0 {
            let directory_data = String::from_utf8_lossy(&buffer[..read_result]).to_string();
            match crate::directory_engine::handle_directory_operations(directory_data) {
                Ok(result) => Ok(result),
                Err(e) => Err(format!("Directory engine error: {}", e))
            }
        } else {
            Err("No directory data received".to_string())
        }
    })
} 