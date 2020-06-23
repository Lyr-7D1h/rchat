use std::io;
use std::io::Read;
use std::io::Write;
use std::net;

pub fn listen(mut stream: &net::TcpStream) -> io::Result<()> {
    let mut buffer = [0; 512];

    loop {
        stream.read(&mut buffer).unwrap();

        println!("Server: {:?}", stream);

        io::empty().read(&mut buffer).unwrap();
    }

    Ok(())
}

pub fn read_input(mut stream: &net::TcpStream) -> io::Result<()> {
    let mut buffer = [0; 512];

    loop {
        io::stdin().read(&mut buffer)?;

        stream.write(&buffer)?;

        io::empty().read(&mut buffer).unwrap();
    }

    Ok(())
}
