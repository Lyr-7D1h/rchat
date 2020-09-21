use std::io;
use std::io::Read;
use std::io::Write;
use std::net;

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
            Ok(message) => match &message {
                Message::Say(msg) => {
                    println!("{} : {}", msg.username, msg.content);
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
    println!("Username: ");
    let mut buffer = String::new();

    io::stdin()
        .read_line(&mut buffer)
        .expect("Could not read input for username");

    let username = buffer.trim().to_string();

    stream
        .write(
            Message::init(&username)
                .expect("Could not create init message")
                .raw(),
        )
        .unwrap();

    stream.flush().unwrap();

    println!("Connected..");

    loop {
        let mut buffer = String::new();

        if let Err(err) = io::stdin().read_line(&mut buffer) {
            eprintln!("{}", err);
            break;
        }

        let input = buffer.trim().to_string();
        match Message::say(&input, &username) {
            Ok(msg) => {
                if let Err(err) = stream.write(msg.raw()) {
                    eprintln!("Dropped io listener: {}", err);
                    break;
                }

                stream.flush().unwrap();
            }
            Err(e) => eprintln!("{}", e),
        }
    }
}
