use crate::websocket::Websocket;
use std::sync::mpsc;
use std::sync::mpsc::Receiver;
use std::thread;

mod request;
mod websocket;

fn main() -> std::io::Result<()> {
    let (stdin_tx, stdin_rx) = mpsc::channel::<String>();

    thread::spawn(move || {
        if let Err(e) = handle_network(stdin_rx) {
            eprintln!("Error while handling socket: {e}")
        }
    });
    loop {
        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Error while reading stdin");
        if input.trim() == "quit" {
            break;
        }
        if let Err(e) = stdin_tx.send(input.clone()) {
            eprintln!("Error: {e}");
        }
    }
    Ok(())
}

fn handle_network(stdin_rx: Receiver<String>) -> std::io::Result<()> {
    let mut ws_socket = Websocket::new("127.0.0.1", 8000);
    ws_socket.init_websocket();
    loop {
        if let Ok(message) = stdin_rx.try_recv() {
            if message == "quit" {
                break;
            }
        }
    }

    Ok(())
}
