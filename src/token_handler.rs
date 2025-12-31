use std::net::TcpStream;
use std::io::Read;

pub fn process_token_stream() -> Result<(), String> {
    let mut stream = TcpStream::connect("127.0.0.1:9094")
        .map_err(|_| "connection failed")?;

    let mut buffer = [0u8; 2048];

    //SOURCE
    let size = stream.read(&mut buffer)
        .map_err(|_| "read failed")?;

    if size == 0 {
        return Err("no data received".to_string());
    }

    let token = String::from_utf8_lossy(&buffer[..size]).to_string();

    crate::token_engine::validate_token(token)
}
