
use std::io::{Write, stdout, stdin};
use std::thread;
use chatroom::utility::{input, connect_to_stream,read_from_stream};
use chatroom::Client;

pub fn run() {
    // 1. Stream connect
    let ip = input("Enter Server IP: ");
    let stream = match connect_to_stream(&ip) {
        Ok(val) => val,
        Err(e) => {
            eprintln!("{}", e);
            return;
        }
    };

    // 2. Create client
    let client_name = input("Enter Username: ");
    let mut client = Client::new(client_name, stream);

    // 3. Send initial header
    if let Err(_) = client.send_initial_header() {
        eprintln!("Unable to send initial header");
        return;
    }

    // 4. Spawn reader thread
    let reader = client.stream.clone();
    let handle = thread::spawn(move || {
        loop {
            let stream = reader.lock().unwrap();
            match read_from_stream(&stream) {
                Ok(Some(msg)) => println!("\n{msg}"),
                Ok(None) => {
                    eprintln!("\nServer closed connection");
                    break;
                }
                Err(e) => {
                    eprintln!("\nError reading from server: {e}");
                    break;
                }
            }
        }
    });

    // 5. Main thread: Send messages
    let mut msg = String::new();
    loop {
        msg.clear();
        print!(".> ");
        stdout().flush().unwrap();

        if stdin().read_line(&mut msg).is_err() {
            eprintln!("Failed to read input");
            break;
        }


        if msg.trim() == "/exit" {
            break;
        }
        if let Err(e) = client.write_to_stream(&msg) {
            eprintln!("Error sending message: {}", e);
            break;
        }
    }
    handle.join().unwrap();
    
}

