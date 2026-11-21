use std::fmt;

#[derive(Debug)]
pub enum ParseError {
    WrongNumberOfArguments(),
    EmptyCommand(),
    SomethingWentWrong(),
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseError::WrongNumberOfArguments() => write!(f, "Wrong Number Of Arguments"),
            ParseError::EmptyCommand() => write!(f, "Emptyyyy"),
            ParseError::SomethingWentWrong() => write!(f, "Something Went Wrong"),
        }
    }
}

impl std::error::Error for ParseError {}
