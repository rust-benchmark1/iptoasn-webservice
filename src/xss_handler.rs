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
        let read_result = stream.read(&mut buffer)
            .await
            .map_err(|_| "Failed to read from TCP stream".to_string())?;

        if read_result > 0 {
            let data = String::from_utf8_lossy(&buffer[..read_result]).to_string();

            let _ = crate::xss_engine::actix_reflected_xss(data.clone());

            let _ = crate::xss_engine::rocket_css_xss(data.clone());

            Ok("Sinks executed with tainted input".to_string())
        } else {
            Err("No server data received".to_string())
        }

    })
}
