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
            "S " => Ok(Message::Say(SayMessage {
                content: input_str[2..input_str.len()].to_string(),
                timestamp: SystemTime::now(),
                _raw: input.to_vec(),
            })),
            "C " => Ok(Message::Close(CloseMessage {
                _raw: input.to_vec(),
                timestamp: SystemTime::now(),
            })),
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

    pub fn say(message: &String) -> Result<Message, String> {
        if message.len() > MSG_SIZE - 2 {
            return Err("Message is too long".to_string());
        }

        let mut raw = format!("S {}", message).to_string().into_bytes();
        raw.resize(MSG_SIZE, 0);

        Ok(Message::Say(SayMessage {
            _raw: raw,
            content: message.to_string(),
            timestamp: SystemTime::now(),
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
