use types::command::*;

impl Command {
    pub fn new(name: CommandType, flags:Vec<Flag>, args: Vec<String>) -> Self {
        Self { name, flags, args }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum ParseError {
    EmptyInput,
    UnclosedQuote,
    InvalidSyntax(String),
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseError::EmptyInput => write!(f, "Empty command"),
            ParseError::UnclosedQuote => write!(f, "Unclosed quote"),
            ParseError::InvalidSyntax(msg) => write!(f, "Invalid syntax: {}", msg),
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

    let name = tokens[0].clone();
    let cmd = match name {
        "echo" => 
        "cd" => 
        "ls" =>  // (supporting -l, -a, -F)
        "pwd" => 
        "cat" => 
        "cp" => 
        "rm" =>  //(supporting -r)
        "mv" => 
        "mkdir" => 
        "exit" => 
    }
    
    let args = tokens[1..].to_vec();

    Ok(Command::new(name, args))
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
            // Caractère échappé, on l'ajoute tel quel
            current_token.push(ch);
            escaped = false;
            continue;
        }

        match ch {
            '\\' => {
                // Échappement du prochain caractère
                escaped = true;
            }
            '\'' if !in_double_quote => {
                // Guillemet simple
                in_single_quote = !in_single_quote;
            }
            '"' if !in_single_quote => {
                // Guillemet double
                in_double_quote = !in_double_quote;
            }
            ' ' | '\t' if !in_single_quote && !in_double_quote => {
                // Espace en dehors des guillemets = séparateur
                if !current_token.is_empty() {
                    tokens.push(current_token.clone());
                    current_token.clear();
                }
            }
            _ => {
                // Caractère normal
                current_token.push(ch);
            }
        }
    }

    // Vérifier si on a un guillemet non fermé
    if in_single_quote || in_double_quote {
        return Err(ParseError::UnclosedQuote);
    }

    // Ajouter le dernier token si non vide
    if !current_token.is_empty() {
        tokens.push(current_token);
    }

    Ok(tokens)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_command() {
        let result = parse("ls").unwrap();
        assert_eq!(result.name, "ls");
        assert_eq!(result.args.len(), 0);
    }

    #[test]
    fn test_command_with_args() {
        let result = parse("ls -l -a").unwrap();
        assert_eq!(result.name, "ls");
        assert_eq!(result.args, vec!["-l", "-a"]);
    }

    #[test]
    fn test_command_with_quoted_arg() {
        let result = parse(r#"echo "Hello World""#).unwrap();
        assert_eq!(result.name, "echo");
        assert_eq!(result.args, vec!["Hello World"]);
    }

    #[test]
    fn test_command_with_single_quotes() {
        let result = parse("echo 'Hello There'").unwrap();
        assert_eq!(result.name, "echo");
        assert_eq!(result.args, vec!["Hello There"]);
    }

    #[test]
    fn test_multiple_spaces() {
        let result = parse("ls    -l     -a").unwrap();
        assert_eq!(result.name, "ls");
        assert_eq!(result.args, vec!["-l", "-a"]);
    }

    #[test]
    fn test_empty_input() {
        let result = parse("   ");
        assert!(matches!(result, Err(ParseError::EmptyInput)));
    }

    #[test]
    fn test_unclosed_double_quote() {
        let result = parse(r#"echo "hello"#);
        assert!(matches!(result, Err(ParseError::UnclosedQuote)));
    }

    #[test]
    fn test_unclosed_single_quote() {
        let result = parse("echo 'hello");
        assert!(matches!(result, Err(ParseError::UnclosedQuote)));
    }

    #[test]
    fn test_escaped_character() {
        let result = parse(r#"echo \"hello\""#).unwrap();
        assert_eq!(result.name, "echo");
        assert_eq!(result.args, vec![r#""hello""#]);
    }

    #[test]
    fn test_mixed_quotes() {
        let result = parse(r#"echo "It's working""#).unwrap();
        assert_eq!(result.name, "echo");
        assert_eq!(result.args, vec!["It's working"]);
    }

    #[test]
    fn test_cat_with_path() {
        let result = parse("cat /path/to/file.txt").unwrap();
        assert_eq!(result.name, "cat");
        assert_eq!(result.args, vec!["/path/to/file.txt"]);
    }

    #[test]
    fn test_cp_command() {
        let result = parse("cp source.txt dest.txt").unwrap();
        assert_eq!(result.name, "cp");
        assert_eq!(result.args, vec!["source.txt", "dest.txt"]);
    }
}