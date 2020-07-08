use rchat::args::Config;
use rchat::client::Client;
use rchat::server::Server;
use std::env;
use std::fmt;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::from(args).unwrap_or_else(|err| {
        eprintln!("{}", err);
        process::exit(1)
    });

    match config {
        Config::Client => {
            let client = Client::new();

            client.connect().unwrap_or_else(print_exit)
        }
        Config::Server => {
            println!("Starting Rex Chat Server");
            let mut server = Server::new().unwrap_or_else(|err| {
                eprintln!("{}", err);
                process::exit(1);
            });

            server.listen().unwrap();
        }
    }
}

fn print_exit<Error: fmt::Debug>(err: Error) {
    eprintln!("{:?}", err);
    process::exit(1);
}
