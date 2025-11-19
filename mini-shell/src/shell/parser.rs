pub struct Command {
    pub name: String,
    pub args: Vec<String>,
}

pub struct Commands {
    pub commands: Vec<Command>,
}

impl Commands {

    pub fn new(commands: Vec<Command>) -> Self {
        Commands { commands }
    }

pub fn parse_commands(input: &str) -> Commands {
    let mut commands = Vec::new();
    let mut current = String::new();
    let mut chars = input.chars().peekable();

    while let Some(c) = chars.next() {
        match c {
            ';' | '\n' => {
                if !current.trim().is_empty() {
                    commands.push(Command::parse_command(&current));
                }
                current.clear();
            }
            '&' => {
                // Check if next char is also &
                if let Some('&') = chars.peek() {
                    chars.next(); // consume second '&'
                    if !current.trim().is_empty() {
                        commands.push(Command::parse_command(&current));
                    }
                    current.clear();
                }
            }
            _ => current.push(c),
        }
    }

    if !current.trim().is_empty() {
        commands.push(Command::parse_command(&current));
    }

    Commands::new(commands)
}


}

impl Command {
    pub fn new(name: String, args: Vec<String>) -> Self {
        Command { name, args }
    }

    pub fn parse_command(input: &str) -> Command {
        let parts: Vec<&str> = input.trim().split_whitespace().collect();
        let name = parts.get(0).unwrap_or(&"").to_string();
        let args = parts.iter().skip(1).map(|s| s.to_string()).collect();
        Command::new(name, args)
    }
}