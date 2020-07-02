use std::io;
use std::io::prelude::*;
use std::io::Read;
use std::net;

pub fn handle_connection(mut stream: net::TcpStream) {
    println!("New client");

    let mut buffer = [0; 512];

    loop {
        stream.read(&mut buffer).unwrap();
        println!("Stream {}", buffer.len());

        match buffer[0] as char {
            'C' => break,
            char => {
                let error_message = if !char.is_alphanumeric() {
                    format!("E No Command given")
                } else {
                    format!("E Undefined Command: {}", char)
                };

                if let Err(err) = stream.write(error_message.as_bytes()) {
                    println!("Client Error: {:?}", err);
                    break;
                }
            }
        }

        io::empty().read(&mut buffer).unwrap();
    }

    println!("Dropped client");
}
