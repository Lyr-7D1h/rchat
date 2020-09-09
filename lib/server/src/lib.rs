use rchat_parser::{parse, Message};
use std::io;
use std::io::prelude::*;
use std::io::Read;
use std::net;
use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use std::thread;

type Clients = Vec<mpsc::Sender<Arc<Message>>>;

pub struct Server {
    clients: Mutex<Clients>,
    listener: net::TcpListener,
}

impl Server {
    pub fn new() -> Result<Server, &'static str> {
        if let Ok(listener) = net::TcpListener::bind("127.0.0.1:7567") {
            let server = Server {
                listener,
                clients: Mutex::new(vec![]),
            };

            Ok(server)
        } else {
            Err("Could not bind to port, there is probably already an instance running.")
        }
    }

    pub fn shutdown(&self) -> io::Result<()> {
        for conn in self.clients.lock().unwrap().iter() {
            return conn
                .send(Arc::new(Message::Close))
                .or_else(|err| Err(io::Error::new(io::ErrorKind::Other, err)));
        }
        Ok(())
    }

    // Listen for new clients
    pub fn listen(self) -> io::Result<()> {
        let (tx, rx) = mpsc::channel();

        let clients = Arc::new(self.clients);

        let c = clients.clone();
        thread::spawn(|| message_listener(c, rx));

        for stream in self.listener.incoming() {
            let stream = stream?;

            let sender = tx.clone();

            let (tx, rx) = mpsc::channel();

            clients.lock().unwrap().push(tx);

            // TODO: fix ownership
            thread::spawn(|| client_listener(stream, sender, rx));
        }

        Ok(())
    }
}

fn message_listener(clients: Arc<Mutex<Clients>>, rx: mpsc::Receiver<Message>) {
    loop {
        let mes = Arc::new(rx.recv().unwrap());

        println!("{:?}", mes);

        for client in clients.lock().unwrap().iter() {
            client.send(mes.clone()).unwrap();
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

    let mut buffer = [0; 512];

    loop {
        if let Ok(mes) = receiver.try_recv() {
            if let Err(err) = stream.write(mes.raw()) {
                //stream.write(buffer.trim().as_bytes()) {
                eprintln!("Dropped io listener: {}", err);
                break;
            }
        }

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
