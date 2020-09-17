use std::time::SystemTime;

const MSG_SIZE: usize = 64;

#[derive(Debug)]
pub enum Message {
    Say(SayMessage),
    Error(ErrorMessage),
    Close(CloseMessage),
}

impl Message {
    pub fn parse(input: &[u8]) -> Result<Message, String> {
        if input.len() < 2 {
            return Err(format!(
                "Input not long enough, it has a lenght of: {}",
                input.len()
            ));
        };

        let input = String::from_utf8_lossy(input);

        match &input[0..2] {
            "E " => Ok(Message::Error(ErrorMessage {
                content: input[2..input.len()].to_string(),
                timestamp: SystemTime::now(),
                _raw: input.to_string(),
            })),
            "S " => Ok(Message::Say(SayMessage {
                content: input[2..input.len()].to_string(),
                timestamp: SystemTime::now(),
                _raw: input.to_string(),
            })),
            "C " => Ok(Message::Close(CloseMessage {
                _raw: input.to_string(),
            })),
            _ => Err(format!("Could not parse: '{}'", input)),
        }
    }

    pub fn close() -> Message {
        Message::Close(CloseMessage {
            _raw: "C".to_string(),
        })
    }

    pub fn say(message: &str) -> Result<Message, String> {
        if message.len() > MSG_SIZE {
            return Err("Message is too long".to_string());
        }
        Ok(Message::Say(SayMessage {
            _raw: format!("S {}", message).to_string(),
            content: message.to_string(),
            timestamp: SystemTime::now(),
        }))
    }

    pub fn raw(&self) -> &[u8] {
        match self {
            Message::Say(msg) => msg._raw.as_bytes(),
            Message::Error(msg) => msg._raw.as_bytes(),
            Message::Close(msg) => msg._raw.as_bytes(),
        }
    }
}

#[derive(Debug)]
pub struct SayMessage {
    pub content: String,
    pub timestamp: SystemTime,
    _raw: String,
}

#[derive(Debug)]
pub struct ErrorMessage {
    pub content: String,
    pub timestamp: SystemTime,
    _raw: String,
}

#[derive(Debug)]
pub struct CloseMessage {
    _raw: String,
}
