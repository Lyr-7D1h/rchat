use rchat_parser::Message;
use std::io;
use std::net;
use std::sync::mpsc;
use std::sync::Arc;
use std::thread;

mod client;
use client::{authenticate, client_listener};

type Clients = Vec<mpsc::Sender<Arc<Message>>>;

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
            // RELAY SAY MESSAGES
            if let Ok(mes) = rx.try_recv() {
                if let Message::Say(_) = mes {
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
            }

            // Check new connections
            if let Ok((stream, socket)) = self.listener.accept() {
                let sender = tx.clone();

                let (tx, rx) = mpsc::channel();

                self.clients.push(tx);

                // TODO: fix ownership}
                thread::spawn(move || {
                    if let Ok(client) = authenticate(stream.try_clone().unwrap(), socket) {
                        println!("New client: {} ({})", client.username, client.socket.ip());

                        client_listener(client, sender, rx)
                    }
                });
            }
            sleep();
        }
    }
}

fn sleep() {
    thread::sleep(::std::time::Duration::from_millis(100))
}
