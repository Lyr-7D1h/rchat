use std::time::SystemTime;

mod message;

pub use message::*;

pub fn parse_string(input: &str) -> Result<Message, String> {
    if input.len() < 3 {
        return Err(format!(
            "Input not long enough, it has a lenght of: {}",
            input.len()
        ));
    };

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
        "C " => Ok(Message::Close {
            _raw: input.to_string(),
        }),
        _ => Err(format!("Could not parse: {}", input)),
    }
}
pub fn parse(input: &[u8; 512]) -> Result<Message, String> {
    if input.len() < 2 {
        return Err(format!(
            "Input not long enough, it has a lenght of: {}",
            input.len()
        ));
    };

    let input = String::from_utf8_lossy(input);
    // let input = input.trim_matches(char::from(0));

    match &input[0..1] {
        "E " => Ok(Message::Error(ErrorMessage {
            content: input[2..input.len()].to_string(),
            timestamp: SystemTime::now(),
            _raw: input.to_string(),
        })),
        "S" => Ok(Message::Say(SayMessage {
            content: input[1..input.len()].to_string(),
            timestamp: SystemTime::now(),
            _raw: input.to_string(),
        })),
        "C" => Ok(Message::Close {
            _raw: input.to_string(),
        }),
        _ => Err(format!("Could not parse: '{}'", input)),
    }
}
