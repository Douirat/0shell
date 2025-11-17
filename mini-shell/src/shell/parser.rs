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
        /*
        ----> Split the input by simicolones or new line && operator for multiple commands:
        */
        let command_strs: Vec<&str> = input.split(|c| c == ';' || c == '&' || c == '\n').collect();
        let commands: Vec<Command> = command_strs
            .iter()
            .map(|&cmd_str| Command::parse_command(cmd_str))
            .collect();
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