use std::io;
use std::io::prelude::*;
use std::io::Read;
use std::net;
use std::sync::mpsc;
use std::thread;

use crate::parser::{parse, Message};

pub struct Connection {
    stream: net::TcpStream,
}

/// Listens for tcp messages parses them and sends them to the given channel
impl Connection {
    pub fn new(stream: net::TcpStream, sender: mpsc::Sender<Message>) -> io::Result<Connection> {
        let s = stream.try_clone()?;

        let conn = Connection { stream };

        thread::spawn(move || {
            tcp_listener(s, sender);
        });

        Ok(conn)
    }

    pub fn shutdown(&self) -> io::Result<()> {
        self.stream.shutdown(net::Shutdown::Both)
    }
}

fn tcp_listener(mut stream: net::TcpStream, sender: mpsc::Sender<Message>) {
    println!("New client");

    let mut buffer = [0; 512];

    loop {
        stream.read(&mut buffer).unwrap();

        let parsed_buffer = parse(&buffer);

        match parsed_buffer {
            Ok(message) => {
                println!("{:?}", message);
                sender.send(message).unwrap_or_else(|err| {
                    eprintln!("{}", err);
                    panic!();
                });
            }
            Err(err) => {
                if let Err(err) = stream.write(err.as_bytes()) {
                    eprintln!("Client Error: {:?}", err);
                    break;
                };
            }
        }

        io::empty().read(&mut buffer).unwrap();
    }

    println!("Dropped client");
}
