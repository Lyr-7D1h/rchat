use std::time::SystemTime;

#[derive(Debug)]
pub enum Message {
    Say(SayMessage),
    Error(ErrorMessage),
    Close(CloseMessage),
}

impl Message {
    pub fn raw(&self) -> &[u8] {
        match self {
            Message::Say(msg) => msg._raw.as_bytes(),
            Message::Error(msg) => msg._raw.as_bytes(),
            Message::Close(msg) => msg._raw.as_bytes(),
        }
    }
}

pub trait MessageType {
    const _raw: String;
    fn raw(&self) -> &[u8] {
        _raw.as_bytes()
    }
}

#[derive(Debug)]
pub struct SayMessage {
    pub content: String,
    pub timestamp: SystemTime,
    pub _raw: String,
}

#[derive(Debug)]
pub struct ErrorMessage {
    pub content: String,
    pub timestamp: SystemTime,
    pub _raw: String,
}

#[derive(Debug)]
pub struct CloseMessage {
    pub _raw: String,
}
