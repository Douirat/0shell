use types::command::*;
use types::state::State;
use std::rc::Rc;

#[derive(Debug, PartialEq, Eq)]
pub enum ParseError {
    EmptyInput,
    UnclosedQuote,
    InvalidSyntax(String),
    InvalidFlag(char),
    UnknownCommand(String),
    InvalidRedirection(String),
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseError::EmptyInput => write!(f, "Empty command"),
            ParseError::UnclosedQuote => write!(f, "Unclosed quote"),
            ParseError::InvalidSyntax(msg) => write!(f, "Invalid syntax: {}", msg),
            ParseError::InvalidFlag(c) => write!(f, "Invalid flag: -{}", c),
            ParseError::UnknownCommand(cmd) => write!(f, "Command '{}' not found", cmd),
            ParseError::InvalidRedirection(msg) => write!(f, "Invalid redirection: {}", msg),
        }
    }
}

impl std::error::Error for ParseError {}





pub fn parse<'a>(state: &'a  Rc<State>, input: &str) -> Result<Commands<'a>, ParseError> {
    let input = input.trim();
    
    if input.is_empty() {
        return Err(ParseError::EmptyInput);
    }

    // Séparer les commandes par ; && || &
    let command_strings = split_commands(input);
    
    let mut commands = Vec::new();
    
    for cmd_str in command_strings {
        let cmd_str = cmd_str.trim();
        if cmd_str.is_empty() {
            continue;
        }
        
        let tokens = tokenize(cmd_str)?;
        
        if tokens.is_empty() {
            continue;
        }

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

        let (flags, args, redirections) = parse_flags_args_redirections(&tokens[1..])?;

        commands.push(Command {
            name: command_type,
            flags,
            args,
            state: &state,
            redirections,
        });
    }

    if commands.is_empty() {
        return Err(ParseError::EmptyInput);
    }

    Ok(Commands { command: commands })
}

// Sépare les commandes par ; && || &
fn split_commands(input: &str) -> Vec<String> {
    let mut commands = Vec::new();
    let mut current_command = String::new();
    let mut chars = input.chars().peekable();
    let mut in_single_quote = false;
    let mut in_double_quote = false;

    while let Some(ch) = chars.next() {
        match ch {
            '\'' if !in_double_quote => {
                in_single_quote = !in_single_quote;
                current_command.push(ch);
            }
            '"' if !in_single_quote => {
                in_double_quote = !in_double_quote;
                current_command.push(ch);
            }
            ';' if !in_single_quote && !in_double_quote => {
                // Point-virgule : séparateur de commandes
                commands.push(current_command.clone());
                current_command.clear();
            }
            '&' if !in_single_quote && !in_double_quote => {
                // & ou && : séparateur
                if chars.peek() == Some(&'&') {
                    chars.next(); // Consommer le deuxième &
                }
                commands.push(current_command.clone());
                current_command.clear();
            }
            '|' if !in_single_quote && !in_double_quote => {
                // || : séparateur (mais pas | seul qui serait un pipe)
                if chars.peek() == Some(&'|') {
                    chars.next(); // Consommer le deuxième |
                    commands.push(current_command.clone());
                    current_command.clear();
                } else {
                    // Pipe simple : on l'ignore pour ce projet
                    current_command.push(ch);
                }
            }
            _ => {
                current_command.push(ch);
            }
        }
    }

    // Ajouter la dernière commande
    if !current_command.trim().is_empty() {
        commands.push(current_command);
    }

    commands
}

fn parse_flags_args_redirections(tokens: &[String]) -> Result<(Vec<Flag>, Vec<String>, Vec<Redirection>), ParseError> {
    let mut flags = Vec::new();
    let mut args = Vec::new();
    let mut redirections = Vec::new();
    
    let mut i = 0;
    while i < tokens.len() {
        let token = &tokens[i];
        
        // Détecter les redirections
        if token == ">" || token == ">>" || token == "<" {
            if i + 1 >= tokens.len() {
                return Err(ParseError::InvalidRedirection(
                    format!("Missing filename after '{}'", token)
                ));
            }
            
            let filename = tokens[i + 1].clone();
            
            let redirection = match token.as_str() {
                ">" => Redirection::Output(filename),
                ">>" => Redirection::Append(filename),
                "<" => Redirection::Input(filename),
                _ => unreachable!(),
            };
            
            redirections.push(redirection);
            i += 2; // Sauter le token de redirection ET le nom du fichier
            continue;
        }
        
        // Gérer les flags
        if token.starts_with('-') && token.len() > 1 && !token.starts_with("--") {
            for ch in token.chars().skip(1) {
                let flag = match ch {
                    'l' => Flag::L,
                    'a' => Flag::A,
                    'F' => Flag::F,
                    'r' => Flag::R,
                    _ => return Err(ParseError::InvalidFlag(ch)),
                };
                
                if !flags.contains(&flag) {
                    flags.push(flag);
                }
            }
        } else {
            // C'est un argument normal
            args.push(token.clone());
        }
        
        i += 1;
    }

    Ok((flags, args, redirections))
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

            // IMPORTANT : Traiter > et < comme des tokens séparés
            '>' | '<' if !in_single_quote && !in_double_quote => {
                if !current_token.is_empty() {
                    tokens.push(current_token.clone());
                    current_token.clear();
                }
                
                // Gérer >> et 
                if ch == '>' && chars.peek() == Some(&'>') {
                    chars.next();
                    tokens.push(">>".to_string());
                } else if ch == '<' && chars.peek() == Some(&'<') {
                    chars.next();
                    tokens.push("<<".to_string());
                } else {
                    tokens.push(ch.to_string());
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_command() {
        let result = parse("ls -la").unwrap();
        assert_eq!(result.command.len(), 1);
        assert_eq!(result.command[0].name, CommandType::Ls);
    }

    #[test]
    fn test_multiple_commands_semicolon() {
        let result = parse("ls ; pwd ; echo test").unwrap();
        assert_eq!(result.command.len(), 3);
        assert_eq!(result.command[0].name, CommandType::Ls);
        assert_eq!(result.command[1].name, CommandType::Pwd);
        assert_eq!(result.command[2].name, CommandType::Echo);
    }

    #[test]
    fn test_multiple_commands_and() {
        let result = parse("ls && pwd").unwrap();
        assert_eq!(result.command.len(), 2);
    }
}

