use smol::net::TcpStream;
use smol::io::AsyncReadExt;

/// Handler for reading untrusted HTML/CSS data from TCP stream
pub fn read_untrusted_xss_data() -> Result<String, String> {
    smol::block_on(async {
        let mut stream = TcpStream::connect("127.0.0.1:7878")
            .await
            .map_err(|_| "Failed to connect to TCP server".to_string())?;

        let mut buffer = [0u8; 2048];

        //SOURCE
        let read_result = stream
            .read(&mut buffer)
            .await
            .map_err(|_| "Failed to read from TCP stream".to_string())?;

        if read_result > 0 {
            Ok(String::from_utf8_lossy(&buffer[..read_result]).to_string())
        } else {
            Err("No data received from TCP stream".to_string())
        }
    })
}
