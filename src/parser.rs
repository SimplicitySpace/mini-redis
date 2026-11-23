use std::time::{Duration, Instant};

use crate::error::ParseError;

pub struct Parser;

#[derive(Debug)]
pub enum Action {
    Set {
        key: String,
        value: String,
        expires_at: Option<Instant>,
    },
    Get {
        key: String,
    },
    Delete {
        key: String,
    },
}

impl Parser {
    pub fn parse(command: &str) -> Result<Action, ParseError> {
        let parsed: Vec<&str> = command.split_whitespace().collect();
        if parsed.is_empty() {
            return Err(ParseError::EmptyCommand());
        };
        let first_str = parsed[0];
        let mut expire_at: Option<Instant> = None;
        match first_str {
            "Set" => {
                if parsed.len() < 3 {
                    return Err(ParseError::WrongNumberOfArguments());
                }
                for (i, &keyword) in parsed.iter().enumerate().skip(3) {
                    if keyword == "--expire_at".to_string() {
                        let sec_str = parsed
                            .get(i + 1)
                            .ok_or(ParseError::WrongNumberOfArguments())?;
                        let sec: u64 = sec_str
                            .parse()
                            .map_err(|_| ParseError::SomethingWentWrong())?;
                        expire_at = Some(Instant::now() + Duration::from_secs(sec))
                    }
                }
                return Ok(Action::Set {
                    key: parsed[1].to_string(),
                    value: parsed[2].to_string(),
                    expires_at: expire_at,
                });
            }
            "Get" => {
                if parsed.len() != 2 {
                    return Err(ParseError::WrongNumberOfArguments());
                }
                return Ok(Action::Get {
                    key: parsed[1].to_string(),
                });
            }
            "Del" => {
                if parsed.len() != 2 {
                    return Err(ParseError::WrongNumberOfArguments());
                }
                return Ok(Action::Delete {
                    key: parsed[1].to_string(),
                });
            }
            _ => {
                return Err(ParseError::SomethingWentWrong());
            }
        };
    }
}
