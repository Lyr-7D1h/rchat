pub fn parse(input: &[u8; 512]) -> Result<Message, String> {
    let input = String::from_utf8_lossy(input);

    match &input[0..1] {
        "E" => Ok(Message::Error {
            content: input[1..input.len()].to_string(),
        }),
        "S" => Ok(Message::Say {
            content: input[1..input.len()].to_string(),
        }),
        "C" => Ok(Message::Close),
        _ => Err(format!("Could not parse: '{}'", input)),
    }
}

#[derive(Debug)]
pub enum Message {
    Say { content: String },
    Error { content: String },
    Close,
}
