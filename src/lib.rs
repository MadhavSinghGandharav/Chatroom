pub mod utility;

use std::collections::HashMap;
use std::io::Write;
use std::net::TcpStream;
use std::sync::{
    Arc, Mutex,
    mpsc::{Receiver, Sender},
};
use utility::{read_from_stream, write_to_stream};

pub struct Client {
    pub username: String,
    pub stream: Arc<Mutex<TcpStream>>,
}

impl Client {
    pub fn new(username: String, stream: TcpStream) -> Self {
        let stream = Arc::new(Mutex::new(stream));
        Self { username, stream }
    }

    pub fn send_initial_header(&mut self) -> std::io::Result<()> {
        let msg = format!("__USERNAME__:{}", self.username);
        let len = msg.len() as u16;

        let mut stream = self.stream.lock().unwrap(); // 🔒 lock here
        stream.write_all(&len.to_be_bytes())?;
        stream.write_all(msg.as_bytes())?;
        Ok(())
    }

    pub fn write_to_stream(&mut self, msg: &str) -> std::io::Result<()> {
        let msg = format!("{}: {}", self.username, msg);
        let len = msg.len() as u16;
        let mut stream = self.stream.lock().unwrap();

        stream.write_all(&len.to_be_bytes())?;
        stream.write_all(msg.as_bytes())?;

        Ok(())
    }
}

pub fn handle_client(
    clients: Arc<Mutex<HashMap<String, Client>>>,
    stream: TcpStream,
    tx: Sender<String>,
) -> Result<(), &'static str> {
    let username_msg = match read_from_stream(&stream) {
        Ok(Some(msg)) => msg,
        Ok(None) => return Err("Server error: Disconnected early"),
        Err(_) => return Err("Server error: Failed to read header"),
    };

    let username = username_msg.trim().replace("__USERNAME__:", "");
    let stream_cloned = stream.try_clone().unwrap();
    let client = Client::new(username.clone(), stream_cloned);

    // Broadcast join message
    let join_msg = format!("{} joined!", &username);
    {
        let clients_guard = clients.lock().unwrap();
        for (_, client) in clients_guard.iter() {
            let _ = write_to_stream(&client.stream.lock().unwrap(), &join_msg);
        }
    }

    {
        let mut clients_guard = clients.lock().unwrap();
        clients_guard.insert(username.clone(), client);
    }

    // Main loop
    loop {
        let msg = match read_from_stream(&stream) {
            Ok(Some(msg)) => msg,
            Ok(None) => return Err("Server error: Disconnected early"),
            Err(_) => return Err("Server error: Failed to read message"),
        };

        let final_msg = format!("{}: {}", username, msg);
        if tx.send(final_msg).is_err() {
            break;
        }
    }

    Ok(())
}
pub fn start_broadcaster(rx: Receiver<String>, clients: Arc<Mutex<HashMap<String, Client>>>) {
    for msg in rx {
        if let Some((sender, _)) = msg.split_once(":") {
            let clients_guard = clients.lock().unwrap();
            for (name, client) in clients_guard.iter() {
                if name != sender.trim() {
                    let _ = write_to_stream(&client.stream.lock().unwrap(), &msg);
                }
            }
        }
    }
}
