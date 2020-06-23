use std::io;
use std::net;
extern crate ctrlc;

pub struct Client {}

mod handler;

impl Client {
    pub fn new() -> Client {
        Client {}
    }

    pub fn connect(&self) -> io::Result<()> {
        let mut stream = net::TcpStream::connect("127.0.0.1:7567")?;

        handler::listen(&stream)?;

        handler::read_input(&stream)?;

        ctrlc::set_handler(move || {
            stream
                .shutdown(net::Shutdown::Both)
                .expect("Shutdown failed");
        })
        .expect("Error setting ctrl-c handler");

        Ok(())
    }
}
