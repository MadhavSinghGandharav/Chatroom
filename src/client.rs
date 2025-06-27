use chatroom::Client;
use chatroom::utility::{connect_to_stream, input, read_from_stream};
use std::io::{Write, stdin, stdout};

use std::thread;

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
    let reader = stream.try_clone().unwrap();
    let client_name = input("Enter Username: ");
    let mut client = Client::new(client_name, stream);
    
    // 3. Send initial header
    if let Err(_) = client.send_initial_header() {
        eprintln!("Unable to send initial header");
        return;
    }

    // 4. Spawn reader thread

    let handle = thread::spawn(move || {
        loop {
            match read_from_stream(&reader) {
                Ok(Some(msg)) => {
                    print!("\r\x1b[2K"); // clear line
                    println!("{msg}");
                    print!(".> ");
                    stdout().flush().unwrap();},
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

        stdin().read_line(&mut msg).expect("");

        if msg.trim() == "/exit" {
            break;
        }
        if let Err(e) = client.write_to_stream(msg.trim()) {
            eprintln!("Error sending message: {}", e);
            break;
        }
    }
    handle.join().unwrap();
}
