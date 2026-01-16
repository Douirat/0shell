use types::command::*;

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

pub fn parse(input: &str) -> Result<Command, ParseError> {
    let input = input.trim();
    
    if input.is_empty() {
        return Err(ParseError::EmptyInput);
    }

    let tokens = tokenize(input)?;
    
    if tokens.is_empty() {
        return Err(ParseError::EmptyInput);
    }

    // Conversion string -> CommandType
    let command_type = match tokens[0].to_lowercase().as_str() {
        "echo" => CommandType::Echo,
        "cd" => CommandType::Cd,
        "ls" => CommandType::Ls,
        "pwd" => CommandType::Pwd,
        "cat" => CommandType::Cat,
        "cp" => CommandType::Cp,
        "rm" => CommandType::Rm,
        "mv" => CommandType::Mv,
        "mkdir" => CommandType::Mkdir,
        "exit" => CommandType::Exit,
        _ => return Err(ParseError::UnknownCommand(tokens[0].clone())),
    };

    // Parser les flags et arguments
    let (flags, args) = parse_flags_and_args(&tokens[1..])?;

    Ok(Command {
        name: command_type,
        flags,
        args,
    })
}

fn parse_flags_and_args(tokens: &[String]) -> Result<(Vec<Flag>, Vec<String>), ParseError> {
    let mut flags = Vec::new();
    let mut args = Vec::new();

    for token in tokens {
        if token.starts_with('-') && token.len() > 1 && !token.starts_with("--") {
            // C'est un flag (ou plusieurs flags combinés comme -la)
            for ch in token.chars().skip(1) {
                // Conversion char -> Flag
                let flag = match ch {
                    'l' => Flag::L,
                    'a' => Flag::A,
                    'F' => Flag::F,
                    'r' => Flag::R,
                    _ => return Err(ParseError::InvalidFlag(ch)),
                };
                
                // Éviter les doublons
                if !flags.contains(&flag) {
                    flags.push(flag);
                }
            }
        } else {
            // C'est un argument normal
            args.push(token.clone());
        }
    }

    Ok((flags, args))
}

fn tokenize(input: &str) -> Result<Vec<String>, ParseError> {
    let mut tokens = Vec::new();
    let mut current_token = String::new();
    let mut chars = input.chars().peekable();
    let mut in_single_quote = false;
    let mut in_double_quote = false;
    let mut escaped = false;

    while let Some(ch) = chars.next() {
        if escaped {
            current_token.push(ch);
            escaped = false;
            continue;
        }

        match ch {
            '\\' => {
                escaped = true;
            }
            '\'' if !in_double_quote => {
                in_single_quote = !in_single_quote;
            }
            '"' if !in_single_quote => {
                in_double_quote = !in_double_quote;
            }
            ' ' | '\t' if !in_single_quote && !in_double_quote => {
                if !current_token.is_empty() {
                    tokens.push(current_token.clone());
                    current_token.clear();
                }
            }
            _ => {
                current_token.push(ch);
            }
        }
    }

    if in_single_quote || in_double_quote {
        return Err(ParseError::UnclosedQuote);
    }

    if !current_token.is_empty() {
        tokens.push(current_token);
    }

    Ok(tokens)
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_simple_command() {
//         let result = parse("ls").unwrap();
//         assert_eq!(result.name, CommandType::Ls);
//         assert_eq!(result.flags.len(), 0);
//         assert_eq!(result.args.len(), 0);
//     }

//     #[test]
//     fn test_command_with_single_flag() {
//         let result = parse("ls -l").unwrap();
//         assert_eq!(result.name, CommandType::Ls);
//         assert_eq!(result.flags, vec![Flag::L]);
//     }

//     #[test]
//     fn test_command_with_combined_flags() {
//         let result = parse("ls -la").unwrap();
//         assert_eq!(result.name, CommandType::Ls);
//         assert!(result.flags.contains(&Flag::L));
//         assert!(result.flags.contains(&Flag::A));
//     }

//     #[test]
//     fn test_command_with_flags_and_args() {
//         let result = parse("ls -l /home").unwrap();
//         assert_eq!(result.name, CommandType::Ls);
//         assert_eq!(result.flags, vec![Flag::L]);
//         assert_eq!(result.args, vec!["/home"]);
//     }

//     #[test]
//     fn test_rm_with_recursive() {
//         let result = parse("rm -r folder").unwrap();
//         assert_eq!(result.name, CommandType::Rm);
//         assert_eq!(result.flags, vec![Flag::R]);
//         assert_eq!(result.args, vec!["folder"]);
//     }

//     #[test]
//     fn test_echo_with_quotes() {
//         let result = parse(r#"echo "Hello World""#).unwrap();
//         assert_eq!(result.name, CommandType::Echo);
//         assert_eq!(result.args, vec!["Hello World"]);
//     }

//     #[test]
//     fn test_invalid_flag() {
//         let result = parse("ls -x");
//         assert!(matches!(result, Err(ParseError::InvalidFlag('x'))));
//     }

//     #[test]
//     fn test_unknown_command() {
//         let result = parse("unknown_cmd");
//         assert!(matches!(result, Err(ParseError::UnknownCommand(_))));
//     }

//     #[test]
//     fn test_empty_input() {
//         let result = parse("   ");
//         assert!(matches!(result, Err(ParseError::EmptyInput)));
//     }
// }