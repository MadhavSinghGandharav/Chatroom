pub mod utility;

use std::{io::{Read, Write}, net::TcpStream};
use std::sync::{Arc,Mutex};

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

        let mut stream = self.stream.lock().unwrap();  // 🔒 lock here
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

