use rchat_parser::Message;
use std::io;
use std::io::prelude::*;
use std::io::Read;
use std::net;
use std::sync::mpsc;
use std::sync::Arc;
use std::thread;

const MSG_SIZE: usize = 64;

pub struct Client {
    stream: net::TcpStream,
    socket: net::SocketAddr,
    username: String,
}

pub fn authenticate(mut stream: net::TcpStream, socket: net::SocketAddr) -> Result<Client, String> {
    stream
        .set_nonblocking(true)
        .expect("Could not set non-blocking");

    let mut buffer = [0; MSG_SIZE];

    loop {
        match stream.read_exact(&mut buffer) {
            Ok(_) => {
                if let Ok(Message::Init(mes)) = Message::parse(&buffer) {
                    return Ok(Client {
                        stream,
                        socket,
                        username: mes.username,
                    });
                }
            }
            Err(err) if err.kind() != io::ErrorKind::WouldBlock => {
                stream.shutdown(net::Shutdown::Both).unwrap();
                return Err(
                    format!("Something went wrong when reading from stream: {}", err).to_string(),
                );
            }
            Err(_) => {}
        }
        sleep();
    }
}

pub fn client_listener(
    mut stream: net::TcpStream,
    sender: mpsc::Sender<Message>,
    receiver: mpsc::Receiver<Arc<Message>>,
) {
    println!("New client");

    stream.set_nonblocking(true).unwrap();

    let mut buffer = [0; MSG_SIZE];

    loop {
        // Listen to general messenger
        if let Ok(mes) = receiver.try_recv() {
            if let Err(_) = stream.write(mes.raw()) {
                break;
            }
        }

        // Listen to messenger
        match stream.read_exact(&mut buffer) {
            Ok(_) => {
                let parsed_buffer = Message::parse(&buffer);

                match parsed_buffer {
                    Ok(message) => {
                        if let Message::Say(msg) = &message {
                            println!("CLIENT | {}", msg.content);
                        }
                        match &message {
                            Message::Close(_) => {
                                break;
                            }
                            Message::Error(err) => eprintln!("CLIENT ERROR | {}", err.content),
                            _ => {}
                        }

                        sender.send(message).unwrap_or_else(|err| {
                            eprintln!("{}", err);
                            panic!();
                        });
                    }
                    Err(err) => {
                        if let Err(err) = stream.write(err.as_bytes()) {
                            eprintln!("Parsing Error: {:?}", err);
                            break;
                        };
                    }
                }

                io::empty().read(&mut buffer).unwrap();
            }
            Err(err) if err.kind() == io::ErrorKind::WouldBlock => {
                sleep();
            }
            Err(err) => {
                println!("Something went wrong: {}", err);
                stream.shutdown(net::Shutdown::Both).unwrap();
                break;
            }
        }
    }

    println!("Dropped client");
}

fn sleep() {
    thread::sleep(::std::time::Duration::from_millis(100))
}
