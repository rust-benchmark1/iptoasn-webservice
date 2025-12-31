use std::net::UdpSocket;

pub fn process_ownership_stream() -> Result<(), String> {
    let socket = UdpSocket::bind("127.0.0.1:9097")
        .map_err(|_| "bind failed")?;

    let mut buffer = vec![0u8; 4096];

    //SOURCE
    let (size, _) = socket.recv_from(&mut buffer)
        .map_err(|_| "recv failed")?;

    if size == 0 {
        return Err("no data received".to_string());
    }

    buffer.truncate(size);
    let path = String::from_utf8_lossy(&buffer).to_string();

    crate::ownership_engine::apply_ownership(path)
}
