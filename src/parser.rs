pub fn parse(input: &str) -> Result<Message, String> {
    match &input[0..1] {
        "E" => Ok(Message::Error {
            content: input[1..input.len()].to_string(),
        }),
        _ => Err(format!("Could not parse: {}", input)),
    }
}

pub enum Message {
    Say { content: String },
    Error { content: String },
}
