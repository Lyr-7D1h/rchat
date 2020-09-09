use std::env;
use std::fmt;
use std::io::{Error, ErrorKind};
use std::process;

use rchat_client::Client;
use rchat_server::Server;

mod args;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = args::Config::from(args).unwrap_or_else(|err| {
        eprintln!("{}", err);
        process::exit(1)
    });

    match config {
        args::Config::Client => {
            let client = Client::new();

            if let Some(err) = client.connect().err() {
                match err.kind() {
                    ErrorKind::ConnectionRefused => {
                        eprintln!("Server is not running or refusing connections");
                        process::exit(1)
                    }
                    _ => {
                        eprintln!("{}", err);
                        process::exit(1)
                    }
                }
            }
        }
        args::Config::Server => {
            println!("Starting Rex Chat Server");
            let mut server = Server::new().unwrap_or_else(|err| {
                eprintln!("{}", err);
                process::exit(1);
            });

            server.listen().unwrap();
        }
    }
}
