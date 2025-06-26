use chatroom::{handle_stream,initialize_stream};
use chatroom::utility::get_ip;
use std::collections::HashMap;
use std::net::TcpListener;
use std::sync::{Mutex,Arc};




pub fn run() {
    let ip = match get_ip() {
        Some(val) => val,
        None => {
            eprintln!("Unable to fetch IP");
            return;
        }
    };

    let listener = match TcpListener::bind("0.0.0.0:8080") {
        Ok(val) => val,
        Err(_) => {
            eprintln!("Unable to bind at port 8080");
            return;
        }
    };
    println!("Server Started at IP: {}", ip);

    let clients = Arc::new(Mutex::new(HashMap::new()));

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let clients = Arc::clone(&clients);
                std::thread::spawn(move || {
                    initialize_stream(Arc::clone(&clients), &stream);
                    handle_client(clients, stream);
                });
            }
            Err(e) => {
                eprintln!("Connection failed: {}", e);
            }
        }
    }
}

