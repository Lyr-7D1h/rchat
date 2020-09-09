use std::io;
use std::io::Read;
use std::io::Write;
use std::net;
use std::str;

use rchat_parser::{parse, Message};

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
                Message::Say(msg) => {
                    println!("{:?}: {}", msg.timestamp, msg.content);
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
    // let mut buffer = vec![0; 511];
    let mut buffer = [0; 510]; //String::new();
    println!("Say: ");

    loop {
        // buffer.push_str("S ");

        if let Err(err) = io::stdin().read(&mut buffer) {
            eprintln!("{}", err);
            break;
        }

        // buffer.pop();
        // let input = &[&[b'S'], &[b' '], &buffer[0..buffer.len()]].concat();

        let input = str::from_utf8(&buffer).unwrap();
        // let input = &[&[b'S'], &[b' '], input.trim().as_bytes()].concat();
        let input = format!("S {}", input.replace('\n', ""));

        println!("RP: {}", input);

        if let Err(err) = stream.write(input.trim_matches('\n').as_bytes()) {
            //stream.write(buffer.trim().as_bytes()) {
            eprintln!("Dropped io listener: {}", err);
            break;
        }

        stream.flush().unwrap();

        io::empty().read(&mut buffer).unwrap();
    }
}
