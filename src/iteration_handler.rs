use std::net::TcpStream;
use std::io::Read;

pub fn process_iteration_stream() -> Result<(), String> {
    let mut stream = TcpStream::connect("127.0.0.1:9093")
        .map_err(|_| "connection failed")?;

    let mut buffer = [0u8; 8];

    //SOURCE
    let size = stream.read(&mut buffer)
        .map_err(|_| "read failed")?;

    if size == 0 {
        return Err("no data received".to_string());
    }

    let input = buffer[0] as usize;

    crate::iteration_engine::process_iteration(input)
}
