#[derive(Debug, PartialEq, Eq)]
pub enum ParseError {
    EmptyInput,
    UnclosedQuote,
    InvalidSyntax(String),
    InvalidFlag(char),
    UnknownCommand(String),
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseError::EmptyInput => write!(f, "Empty command"),
            ParseError::UnclosedQuote => write!(f, "Unclosed quote"),
            ParseError::InvalidSyntax(msg) => write!(f, "Invalid syntax: {}", msg),
            ParseError::InvalidFlag(c) => write!(f, "Invalid flag: -{}", c),
            ParseError::UnknownCommand(cmd) => write!(f, "Command '{}' not found", cmd),
        }
    }
}

impl std::error::Error for ParseError {}