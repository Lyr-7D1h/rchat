use rchat_parser::Message;
use std::io;
use std::io::prelude::*;
use std::io::Read;
use std::net;
use std::sync::mpsc;
use std::sync::Arc;
use std::thread;

type Clients = Vec<mpsc::Sender<Arc<Message>>>;

const MSG_SIZE: usize = 64;

pub struct Server {
    clients: Clients,
    listener: net::TcpListener,
}

impl Server {
    pub fn new() -> Result<Server, &'static str> {
        if let Ok(listener) = net::TcpListener::bind("127.0.0.1:7567") {
            let server = Server {
                listener,
                clients: vec![],
            };

            Ok(server)
        } else {
            Err("Could not bind to port, there is probably already an instance running.")
        }
    }

    pub fn shutdown(&self) -> io::Result<()> {
        for conn in self.clients.iter() {
            return conn
                .send(Arc::new(Message::close()))
                .or_else(|err| Err(io::Error::new(io::ErrorKind::Other, err)));
        }
        Ok(())
    }

    // Listen for new clients
    pub fn listen(mut self) -> io::Result<()> {
        let (tx, rx) = mpsc::channel();

        // let clients = Arc::new(self.clients);

        self.listener
            .set_nonblocking(true)
            .expect("Could not set listener to non blocking");

        loop {
            // Check for general messages
            if let Ok(mes) = rx.try_recv() {
                let mes: Arc<Message> = Arc::new(mes);

                // remove bad clients
                self.clients.retain(|client| {
                    if let Err(err) = client.send(mes.clone()) {
                        eprintln!("{}", err);
                        false
                    } else {
                        true
                    }
                });
            }

            // Check new connections
            if let Ok((stream, _socket)) = self.listener.accept() {
                let sender = tx.clone();

                let (tx, rx) = mpsc::channel();

                self.clients.push(tx);

                // TODO: fix ownership
                thread::spawn(|| client_listener(stream, sender, rx));
            }
            sleep();
        }
    }
}

fn client_listener(
    mut stream: net::TcpStream,
    sender: mpsc::Sender<Message>,
    receiver: mpsc::Receiver<Arc<Message>>,
) {
    println!("New client");

    stream.set_nonblocking(true).unwrap();

    loop {
        if let Ok(mes) = receiver.try_recv() {
            if let Err(err) = stream.write(mes.raw()) {
                //stream.write(buffer.trim().as_bytes()) {
                eprintln!("Dropped io listener: {}", err);
                break;
            }
        }

        let mut buffer = [0; MSG_SIZE];

        match stream.read_exact(&mut buffer) {
            Ok(_) => {
                let parsed_buffer = Message::parse(&buffer);

                match parsed_buffer {
                    Ok(message) => {
                        if let Message::Say(msg) = &message {
                            println!("CLIENT | {}", msg.content);
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
