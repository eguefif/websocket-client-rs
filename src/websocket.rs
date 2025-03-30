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
        let key = self.send_http_hand_shake();
        if let Some(response) = self.read_handshake_response() {
            if !check_server_handshake(key, response) {
                panic!("Error: Server Sec-WebSocket-Accept is incorrect ");
            }
            println!("Handshake done");
        } else {
            panic!("Error in server response")
        }
    }

    fn send_http_hand_shake(&mut self) -> String {
        let key = BASE64_STANDARD.encode(generate_key());
        let header = get_header(key.clone());
        self.socket.write_all(header.as_bytes()).unwrap();
        key
    }

    fn read_handshake_response(&mut self) -> Option<Request> {
        let mut buffer = vec![0; 1024];
        let mut response = String::new();
        loop {
            if let Ok(_) = self.socket.read(&mut buffer) {
                let chunk = String::from_utf8_lossy(&buffer);
                if chunk.contains("\r\n\r\n") {
                    let chunk = chunk.split("\r\n\r\n").next().unwrap();
                    response.push_str(&chunk);
                    return Some(Request::new(&response));
                }
                response.push_str(&chunk);
            } else {
                break;
            }
        }
        None
    }
}

fn transform_key(key: &str) -> String {
    let guid = "258EAFA5-E914-47DA-95CA-C5AB0DC85B11";
    let mut hasher = Sha1::new();
    hasher.update(format!("{key}{guid}"));
    let hash = hasher.finalize();
    BASE64_STANDARD.encode(hash)
}
fn generate_key() -> String {
    let mut rng = rand::rng();
    let mut key = String::new();

    for _ in 0..16 {
        key.push(rng.random::<char>());
    }
    key
}

fn get_header(key: String) -> String {
    let mut header = String::new();
    header.push_str("GET / HTTP1.1\r\n");
    header.push_str("Host: 127.0.0.1\r\n");
    header.push_str("Upgrade: websocket\r\n");
    header.push_str("Connection: Upgrade\r\n");
    header.push_str("Sec-WebSocket-Key: ");
    header.push_str(key.as_str());
    header.push_str("\r\n");
    header.push_str("Sec-WebSocket-Version: 13\r\n");
    header.push_str("\r\n");
    header.push_str("\r\n");
    header
}

fn check_server_handshake(key: String, request: Request) -> bool {
    if let Some(server_key) = request.get_value("Sec-WebSocket-Accept") {
        let control_key = transform_key(&key);
        println!("server key: {}", server_key);
        println!("control key: {}", control_key);
        return server_key == control_key;
    }
    false
}
