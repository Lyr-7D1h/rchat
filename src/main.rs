use rchat::args::Config;
use rchat::server::Server;
use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::from(args).unwrap_or_else(|err| {
        eprintln!("{}", err);
        process::exit(1)
    });

    match config {
        Config::Client => {}
        Config::Server => {
            println!("Starting Rex Chat Server");
            let server = Server::new().unwrap();
            server.listen().unwrap();
        }
    }
}
