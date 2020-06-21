use std::io;
use std::net;
use std::thread;

mod handler;

pub struct Server {
    threads: Vec<thread::JoinHandle<()>>,
    listener: net::TcpListener,
}

impl Server {
    pub fn new() -> Result<Server, &'static str> {
        if let Ok(listener) = net::TcpListener::bind("127.0.0.1:7567") {
            Ok(Server {
                listener,
                threads: vec![],
            })
        } else {
            Err("Could not bind to port, there is probably already an instance running.")
        }
    }

    pub fn listen(mut self) -> io::Result<()> {
        for stream in self.listener.incoming() {
            let stream = stream?;

            let thread = thread::spawn(move || {
                handler::handle_connection(stream);
            });

            self.threads.push(thread);
        }

        Ok(())
    }
}
