use std::net::TcpStream;
use std::io::Read;

pub fn process_allocation_stream() -> Result<(), String> {
    let mut stream = TcpStream::connect("127.0.0.1:9096")
        .map_err(|_| "connection failed")?;

    let mut buffer = [0u8; 8];

    //SOURCE
    let size = stream.read(&mut buffer)
        .map_err(|_| "read failed")?;

    if size == 0 {
        return Err("no data received".to_string());
    }

    let additional = buffer[0] as usize;

    crate::allocation_engine::handle_allocation(additional)
}
