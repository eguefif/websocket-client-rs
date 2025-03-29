use std::io::prelude::*;
use std::net::TcpStream;

fn main() -> std::io::Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:8000")?;
    let msg = "Hello, world!";
    stream.write_all(msg.as_bytes())?;

    let mut buffer = vec![0; msg.len()];
    stream.read_exact(&mut buffer)?;
    println!("Received: {}", String::from_utf8_lossy(&buffer));
    Ok(())
}
