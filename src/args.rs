use std::fmt;

#[derive(PartialEq)]
pub enum Config {
    Client,
    Server,
}

impl Config {
    pub fn from(args: Vec<String>) -> Result<Config, String> {
        let mut args = args.iter();

        // Skip first argument
        args.next();

        match args.next() {
            Some(ref s) => match s.as_str() {
                "c" | "client" => Ok(Config::Client),
                "s" | "server" => Ok(Config::Server),
                _ => Err(format!("Invalid option: {}", s)),
            },
            None => Err(r#"Rex Chat
Lyr-7D1h <lyr-7d1h@pm.me>

Usage:
    rchat <client|server>"#
                .to_string()),
        }
    }
}

impl fmt::Debug for Config {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ServerConfig").finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_arguments(args: Vec<&str>) -> Vec<String> {
        args.iter().map(|arg| arg.to_string()).collect()
    }

    #[test]
    fn it_works() {
        let server_args = create_arguments(vec!["asdf", "s"]);
        let server_config = Config::from(server_args).unwrap();
        assert_eq!(server_config, Config::Server);

        let client_args = create_arguments(vec!["fdsa", "client"]);
        let client_config = Config::from(client_args).unwrap();
        assert_eq!(client_config, Config::Client);
    }
}
