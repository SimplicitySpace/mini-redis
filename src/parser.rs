use crate::error::ParseError;

pub struct Parser;

#[derive(Debug)]
pub enum Action {
    Set { key: String, value: String },
    Get { key: String },
    Delete { key: String },
}

impl Parser {
    pub fn parse(command: &str) -> Result<Action, ParseError> {
        let parsed: Vec<&str> = command.split_whitespace().collect();
        if parsed.is_empty() {
            return Err(ParseError::EmptyCommand());
        };
        let first_str = parsed[0];
        match first_str {
            "Set" => {
                if parsed.len() != 3 {
                    return Err(ParseError::WrongNumberOfArguments());
                }
                return Ok(Action::Set {
                    key: parsed[1].to_string(),
                    value: parsed[2].to_string(),
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
