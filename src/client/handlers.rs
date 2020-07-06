use std::io;
use std::io::Read;
use std::io::Write;
use std::net;

use crate::parser::{parse, Message};

pub fn listen(mut stream: net::TcpStream) {
    let mut buffer = [0; 512];

    loop {
        if let Err(err) = stream.read(&mut buffer) {
            eprintln!("Dropped listener: {}", err);
            break;
        }

        let parsed_buffer = parse(&buffer);

        match parsed_buffer {
            Ok(message) => match message {
                Message::Say { content } => {
                    // println!("Server: {}", content);
                }
                _ => {}
            },
            Err(err) => {} //eprintln!("{}", err),
        }

        io::empty().read(&mut buffer).unwrap();
    }
}

pub fn read_input(mut stream: net::TcpStream) {
    let mut buffer = vec![0; 511];

    loop {
        io::stdin().read(&mut buffer).unwrap();

        // println!("{:?}", buffer);
        buffer.pop();
        let input = &[&[b'S'], &buffer[0..buffer.len()]].concat();

        if let Err(err) = stream.write(&input) {
            eprintln!("Dropped io listener: {}", err);
            break;
        }

        io::empty().read(&mut buffer).unwrap();
    }
}
