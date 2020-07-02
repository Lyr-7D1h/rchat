extern crate ctrlc;
use std::io;
use std::net;
use std::sync::{Arc, Mutex};
use std::thread;

pub struct Client {}

mod handlers;

impl Client {
    pub fn new() -> Client {
        Client {}
    }

    pub fn connect(&self) -> io::Result<()> {
        let stream = net::TcpStream::connect("127.0.0.1:7567")?;

        let stream = Arc::new(Mutex::new(stream));

        let mut threads = vec![];

        // {
        //     let stream = Arc::clone(&stream);

        //     let handler = thread::spawn(move || {
        //         handlers::listen(stream).expect("Stopped listening to server.")
        //     });

        //     threads.push(handler);
        // }

        {
            let stream = Arc::clone(&stream);

            println!("Spawning input reader");
            let handler = thread::spawn(move || {
                handlers::read_input(stream) //.expect("Stopped reading input.")
            });

            threads.push(handler);
        }

        ctrlc::set_handler(move || {
            stream
                .lock()
                .unwrap()
                .shutdown(net::Shutdown::Both)
                .expect("Shutdown failed");
        })
        .expect("Error setting ctrl-c handler");

        for handle in threads {
            handle.join().unwrap()?;
        }

        Ok(())
    }
}
