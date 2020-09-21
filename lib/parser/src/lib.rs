use std::time::SystemTime;

const MSG_SIZE: usize = 64;

#[derive(Debug)]
pub enum Message {
    Say(SayMessage),
    Error(ErrorMessage),
    Close(CloseMessage),
    Init(InitMessage),
}

impl Message {
    pub fn parse(input: &[u8]) -> Result<Message, String> {
        if input.len() < 2 {
            return Err(format!(
                "Input not long enough, it has a lenght of: {}",
                input.len()
            ));
        };

        let input_str = String::from_utf8_lossy(input);

        match &input_str[0..2] {
            "E " => Ok(Message::Error(ErrorMessage {
                content: input_str[2..input_str.len()].to_string(),
                timestamp: SystemTime::now(),
                _raw: input.to_vec(),
            })),
            "S " => {
                let mut input_str = input_str[2..input_str.len()].split_whitespace();

                if let Some(username) = input_str.next() {
                    let mut content = String::new();

                    while let Some(x) = input_str.next() {
                        content.push_str(x);
                        content.push(' ');
                    }

                    input_str.for_each(|x| {
                        content.push(' ');
                        content.push_str(x)
                    });

                    Ok(Message::Say(SayMessage {
                        content,
                        username: username.to_string(),
                        timestamp: SystemTime::now(),
                        _raw: input.to_vec(),
                    }))
                } else {
                    Err(String::from("Could not get username"))
                }
            }
            "C " => Ok(Message::Close(CloseMessage {
                _raw: input.to_vec(),
                timestamp: SystemTime::now(),
            })),
            "I " => {
                let username = input_str[2..input_str.len()].to_string();

                Ok(Message::Init(InitMessage {
                    _raw: input.to_vec(),
                    timestamp: SystemTime::now(),
                    username,
                }))
            }
            _ => Err(format!("Could not parse: '{}'", input_str)),
        }
    }

    pub fn close() -> Message {
        let mut raw = "C".to_string().into_bytes();
        raw.resize(MSG_SIZE, 0);

        Message::Close(CloseMessage {
            _raw: raw,
            timestamp: SystemTime::now(),
        })
    }

    pub fn error(message: &str) -> Result<Message, String> {
        if message.len() > MSG_SIZE - 2 {
            return Err("Message is too long".to_string());
        }

        let mut raw = format!("E {}", message).to_string().into_bytes();
        raw.resize(MSG_SIZE, 0);

        Ok(Message::Error(ErrorMessage {
            _raw: raw,
            timestamp: SystemTime::now(),
            content: message.to_string(),
        }))
    }

    pub fn say(message: &String, username: &String) -> Result<Message, String> {
        if message.len() > MSG_SIZE - 2 - username.len() {
            return Err("Message is too long".to_string());
        }

        let mut raw = format!("S {} {}", username, message)
            .to_string()
            .into_bytes();
        raw.resize(MSG_SIZE, 0);

        Ok(Message::Say(SayMessage {
            _raw: raw,
            username: username.to_string(),
            content: message.to_string(),
            timestamp: SystemTime::now(),
        }))
    }

    pub fn init(username: &String) -> Result<Message, String> {
        let mut raw = format!("I {}", username).to_string().into_bytes();
        raw.resize(MSG_SIZE, 0);

        Ok(Message::Init(InitMessage {
            _raw: raw,
            timestamp: SystemTime::now(),
            username: username.to_string(),
        }))
    }

    pub fn timestamp(&self) -> SystemTime {
        match self {
            Message::Say(msg) => msg.timestamp,
            Message::Error(msg) => msg.timestamp,
            Message::Close(msg) => msg.timestamp,
            Message::Init(msg) => msg.timestamp,
        }
    }

    pub fn raw(&self) -> &Vec<u8> {
        match self {
            Message::Say(msg) => &msg._raw,
            Message::Error(msg) => &msg._raw,
            Message::Close(msg) => &msg._raw,
            Message::Init(msg) => &msg._raw,
        }
    }
}

#[derive(Debug)]
pub struct SayMessage {
    _raw: Vec<u8>,
    timestamp: SystemTime,
    pub username: String,
    pub content: String,
}

#[derive(Debug)]
pub struct ErrorMessage {
    _raw: Vec<u8>,
    timestamp: SystemTime,
    pub content: String,
}

#[derive(Debug)]
pub struct CloseMessage {
    _raw: Vec<u8>,
    timestamp: SystemTime,
}

#[derive(Debug)]
pub struct InitMessage {
    _raw: Vec<u8>,
    timestamp: SystemTime,
    // pub secret: String,
    pub username: String, // Lowercase
}
