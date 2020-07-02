use std::io;
use std::io::Read;
use std::io::Write;
use std::net;
use std::sync::{Arc, Mutex};

type ThreadedStream = Arc<Mutex<net::TcpStream>>;

pub fn listen(stream: ThreadedStream) -> io::Result<()> {
    let mut buffer = [0; 512];

    loop {
        stream.lock().unwrap().read(&mut buffer).unwrap();

        println!("Server: {:?}", stream);

        io::empty().read(&mut buffer).unwrap();
    }

    Ok(())
}

pub fn read_input(stream: ThreadedStream) -> io::Result<()> {
    let mut buffer = [0; 512];

    loop {
        io::stdin().read(&mut buffer)?;

        println!("Writing to server {:?}", buffer.to_ascii_lowercase());

        stream.lock().unwrap().write(&buffer)?;

        io::empty().read(&mut buffer).unwrap();
    }

    Ok(())
}
