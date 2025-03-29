use crate::request::Request;
use base64::prelude::*;
use rand::Rng;
use sha1::{Digest, Sha1};
use std::io::prelude::*;
use std::net::TcpStream;

#[allow(dead_code)]
pub struct Websocket {
    socket: TcpStream,
}

impl Websocket {
    pub fn new(ip: &str, port: i64) -> Self {
        let interface = format!("{ip}:{}", port.to_string());
        let stream = TcpStream::connect(interface).expect("Impossible to open a socket TCP");
        Self { socket: stream }
    }

    pub fn init_websocket(&mut self) {
        let key = make_key(&mut self.socket);
        let mut response = String::new();
        let _ = self.read_socket(&mut response);
        let request = Request::new(&response);
        println!("Request: {:?}", request);
        if !check_server_handshake(key, request) {
            panic!("Server handshake is incorrect");
        }
        println!("Handshake done");
    }

    fn read_socket(&mut self, response: &mut String) -> usize {
        let mut n = 0;
        let mut buffer = vec![0; 1024];
        loop {
            if let Ok(ret) = self.socket.read(&mut buffer) {
                n += ret;
                let chunk = String::from_utf8_lossy(&buffer);
                if chunk.contains("\r\n\r\n") {
                    let chunk = chunk.split("\r\n\r\n").next().unwrap();
                    response.push_str(&chunk);
                    return n;
                }
                response.push_str(&chunk);
            } else {
                break;
            }
        }
        n
    }
}

fn check_server_handshake(key: String, request: Request) -> bool {
    if let Some(server_key) = request.get_value("Sec-WebSocket-Key") {
        let transformed_key = transform_key(&key);
        return server_key == transformed_key;
    }
    false
}

fn transform_key(key: &str) -> String {
    let mut hasher = Sha1::new();
    hasher.update(key);
    let hash = hasher.finalize();
    BASE64_STANDARD.encode(hash)
}

fn make_key(stream: &mut TcpStream) -> String {
    let key = BASE64_STANDARD.encode(generate_key());
    let header = get_header(key.clone());
    stream.write_all(header.as_bytes()).unwrap();
    key
}

fn get_header(key: String) -> String {
    let mut header = String::new();
    header.push_str("GET / HTTP1.1");
    header.push_str("\r\n");
    header.push_str("Host: 127.0.0.1");
    header.push_str("\r\n");
    header.push_str("Upgrade: websocket");
    header.push_str("\r\n");
    header.push_str("Connection: Upgrade");
    header.push_str("\r\n");
    header.push_str("Sec-WebSocket-Key: ");
    header.push_str(key.as_str());
    header.push_str("\r\n");
    header.push_str("Sec-WebSocket-Version: 13");
    header.push_str("\r\n");
    header.push_str("\r\n");
    header
}

fn generate_key() -> String {
    let mut rng = rand::rng();
    let mut key = String::new();

    for _ in 0..16 {
        key.push(rng.random::<char>());
    }
    key
}
