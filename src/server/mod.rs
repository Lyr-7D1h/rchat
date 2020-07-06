use connection::Connection;
use std::io;
use std::net;
use std::sync::mpsc;
use std::thread;

use crate::Message;

mod connection;

pub struct Server {
    connections: Vec<Connection>,
    listener: net::TcpListener,
}

impl Server {
    pub fn new() -> Result<Server, &'static str> {
        if let Ok(listener) = net::TcpListener::bind("127.0.0.1:7567") {
            let server = Server {
                listener,
                connections: vec![],
            };

            Ok(server)
        } else {
            Err("Could not bind to port, there is probably already an instance running.")
        }
    }

    pub fn shutdown(&self) -> io::Result<()> {
        for conn in self.connections.iter() {
            println!("Closing connection");
            conn.shutdown()?;
        }
        Ok(())
    }

    pub fn listen(&mut self) -> io::Result<()> {
        let (tx, rx) = mpsc::channel();

        thread::spawn(|| message_listener(rx));

        for stream in self.listener.incoming() {
            let stream = stream?;

            let tx = tx.clone();

            let conn = Connection::new(stream, tx)?;

            self.connections.push(conn);
        }

        Ok(())
    }
}

fn message_listener(rx: mpsc::Receiver<Message>) {
    let mes = rx.recv().unwrap();

    println!("{:?}", mes);
}
