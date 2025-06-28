use chatroom::utility::get_ip;
use chatroom::{handle_client, start_broadcaster};
use std::collections::HashMap;
use std::net::TcpListener;

use std::sync::{Arc, Mutex, mpsc};

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
    let (tx, rx) = mpsc::channel();
    let clients_cloned = Arc::clone(&clients);
    let handle = std::thread::spawn(move || {
        start_broadcaster(rx, clients_cloned);
    });
    // waiting for client
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                // spawing individual thread for every user

                let clients_cloned = Arc::clone(&clients);
                let tx_cloned = tx.clone();

                std::thread::spawn(move || {
                    if let Err(_) = handle_client(clients_cloned, stream, tx_cloned){
                        return;
                    }
                });
            }
            Err(e) => {
                eprintln!("Connection failed: {}", e);
            }
        }
    }
    handle.join().unwrap();
}
