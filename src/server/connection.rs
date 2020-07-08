use std::io;
use std::net;
use std::sync::mpsc;

use crate::parser::{parse, Message};

pub struct Connection {
    pub stream: net::TcpStream,
    pub sender: mpsc::Sender<Message>,
}

/// Listens for tcp messages parses them and sends them to the given channel
impl Connection {
    pub fn new(stream: net::TcpStream, sender: mpsc::Sender<Message>) -> io::Result<Connection> {
        let s = stream.try_clone()?;

        let conn = Connection { stream, sender };

        Ok(conn)
    }

    pub fn shutdown(&self) -> io::Result<()> {
        self.stream.shutdown(net::Shutdown::Both)
    }
}
