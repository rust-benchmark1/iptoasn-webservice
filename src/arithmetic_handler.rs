use std::net::UdpSocket;

pub fn process_arithmetic_stream() -> Result<(), String> {
    let socket = UdpSocket::bind("127.0.0.1:9095")
        .map_err(|_| "bind failed")?;

    let mut buffer = [0u8; 8];

    //SOURCE
    let (size, _) = socket.recv_from(&mut buffer)
        .map_err(|_| "recv failed")?;

    if size == 0 {
        return Err("no data received".to_string());
    }

    let b = buffer[0] as i32;

    crate::arithmetic_engine::process_arithmetic(b)?;
    Ok(())
}
