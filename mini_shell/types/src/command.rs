// A command enumeration to limit the falling input and redirect the distinction:
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CommandType {
    Echo,
    Cd,
    Ls, //(supporting -l, -a, -F)
    Pwd,
    Cat,
    Cp,
    Rm, // (supporting -r)
    Mv,
    Mkdir,
    Exit,
    Unknown(String), // Pour les commandes non reconnues
}

impl CommandType {
    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
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
            _ => CommandType::Unknown(s.to_string()),
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            CommandType::Echo => "echo",
            CommandType::Cd => "cd",
            CommandType::Ls => "ls",
            CommandType::Pwd => "pwd",
            CommandType::Cat => "cat",
            CommandType::Cp => "cp",
            CommandType::Rm => "rm",
            CommandType::Mv => "mv",
            CommandType::Mkdir => "mkdir",
            CommandType::Exit => "exit",
            CommandType::Unknown(s) => s,
        }
    }
}

// Enumeration to control the flags of ls and rm;
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Flag {
    L, // -l
    A, // -a
    F, // -F
    R, // -r
}

impl Flag {
    pub fn from_char(c: char) -> Option<Self> {
        match c {
            'l' => Some(Flag::L),
            'a' => Some(Flag::A),
            'F' => Some(Flag::F),
            'r' => Some(Flag::R),
            _ => None,
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            Flag::L => "-l",
            Flag::A => "-a",
            Flag::F => "-F",
            Flag::R => "-r",
        }
    }
}

// Packaging commands together to ease multiple command handling:
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Commands {
    pub command: Vec<Command>,
}

// Command representer: {name: CommandType, flags: Vec<Flag>, args: Vec<String>}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Command {
    pub name: CommandType,
    pub flags: Vec<Flag>,
    pub args: Vec<String>,
}

impl Command {
    pub fn new(name: CommandType, flags: Vec<Flag>, args: Vec<String>) -> Self {
        Self { name, flags, args }
    }

    pub fn has_flag(&self, flag: &Flag) -> bool {
        self.flags.contains(flag)
    }
}