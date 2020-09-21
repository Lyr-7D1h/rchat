use std::io;
use std::io::{Read, Write};
use std::net;
use std::thread;

use rchat_parser::Message;

pub struct Client {}

mod handlers;

impl Client {
    pub fn new() -> Client {
        Client {}
    }

    pub fn connect(&self) -> io::Result<()> {
        let stream = net::TcpStream::connect("127.0.0.1:7567")?;

        let mut threads = vec![];

        {
            let stream = stream.try_clone().unwrap();

            let handler = thread::spawn(move || {
                handlers::listen(stream);
            });

            threads.push(handler);
        }

        {
            let stream = stream.try_clone().unwrap();

            let handler = thread::spawn(move || handlers::read_input(stream));

            threads.push(handler);
        }

        for handle in threads {
            handle.join().unwrap();
        }

        Ok(())
    }
}
