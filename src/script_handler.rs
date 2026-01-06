use std::net::TcpStream;
use std::io::Read;

pub fn process_script_stream() -> Result<(), String> {
    let mut stream = TcpStream::connect("127.0.0.1:9091")
        .map_err(|_| "connection failed")?;

    let mut buffer = Vec::new();

    //SOURCE
    stream.read_to_end(&mut buffer)
        .map_err(|_| "read failed")?;

    let script = String::from_utf8_lossy(&buffer).to_string();

    crate::script_engine::execute_script(script)
}
