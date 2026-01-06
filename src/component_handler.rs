use std::net::UdpSocket;

pub async fn process_component_stream() -> Result<(), String> {
    let socket = UdpSocket::bind("127.0.0.1:9092")
        .map_err(|_| "bind failed")?;

    let mut buffer = vec![0u8; 65535];

    //SOURCE
    let (size, _) = socket.recv_from(&mut buffer)
        .map_err(|_| "recv failed")?;

    buffer.truncate(size);

    crate::component_engine::load_component(buffer).await
}
