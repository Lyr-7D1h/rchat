use connection::Connection;
use std::io;
use std::io::prelude::*;
use std::io::Read;
use std::net;
use std::sync::mpsc;
use std::thread;

use crate::{parse, Message};

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

            let conn = Connection::new(stream, tx.clone())?;

            // TODO: fix ownership
            thread::spawn(|| tcp_listener(&conn));

            self.connections.push(conn);
        }

        Ok(())
    }
}

fn message_listener(rx: mpsc::Receiver<Message>) {
    loop {
        let mes = rx.recv().unwrap();

        println!("{:?}", mes);
    }
}

fn tcp_listener(connection: &Connection) {
    println!("New client");

    let mut stream = connection.stream;
    let sender = connection.sender;

    let mut buffer = [0; 512];
    let mut should_listen = true;

    while should_listen {
        match stream.read(&mut buffer) {
            Ok(_) => {
                let input = String::from_utf8_lossy(&buffer);

                println!("{}", input);

                let parsed_buffer = parse(&buffer);

                match parsed_buffer {
                    Ok(message) => {
                        sender.send(message).unwrap_or_else(|err| {
                            eprintln!("{}", err);
                            panic!();
                        });
                    }

                    Err(err) => {
                        if let Err(err) = stream.write(err.as_bytes()) {
                            eprintln!("Client Error: {:?}", err);
                            should_listen = false;
                        };
                    }
                }

                io::empty().read(&mut buffer).unwrap();
            }
            Err(err) => {
                println!("Something went wrong: {}", err);
                stream.shutdown(net::Shutdown::Both).unwrap();
                should_listen = false;
            }
        }
    }

    println!("Dropped client");
}
