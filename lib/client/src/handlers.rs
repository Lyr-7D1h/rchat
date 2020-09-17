use std::io;
use std::io::Read;
use std::io::Write;
use std::net;
use std::str;

const MSG_SIZE: usize = 64;

use rchat_parser::Message;

pub fn listen(mut stream: net::TcpStream) {
    let mut buffer = [0; MSG_SIZE];

    loop {
        if let Err(err) = stream.read_exact(&mut buffer) {
            eprintln!("Dropped listener: {}", err);
            break;
        }

        let parsed_buffer = Message::parse(&buffer);

        match parsed_buffer {
            Ok(message) => match message {
                Message::Say(msg) => {
                    // println!("{:?}", msg.content);
                    println!("SERVER | {:?}: {}", msg.timestamp, msg.content);
                }
                _ => {}
            },
            Err(err) => {
                eprintln!("{}", err);
            }
        }

        io::empty().read(&mut buffer).unwrap();
    }
}

pub fn read_input(mut stream: net::TcpStream) {
    let mut buffer = [0; MSG_SIZE];

    loop {
        if let Err(err) = io::stdin().read(&mut buffer) {
            eprintln!("{}", err);
            break;
        }

        let input = str::from_utf8(&buffer).unwrap().replace("\n", "");
        match Message::say(&input) {
            Ok(msg) => {
                // println!("{:?}", msg.raw());
                if let Err(err) = stream.write(msg.raw()) {
                    eprintln!("Dropped io listener: {}", err);
                    break;
                }

                stream.flush().unwrap();

                io::empty().read(&mut buffer).unwrap();
            }
            Err(e) => eprintln!("{}", e),
        }
    }
}
