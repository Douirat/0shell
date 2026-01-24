// A command enumeration to limit the falling input and redirect the distinction:
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CommandType<T>{
Echo,
Cd,
Ls(T), //(supporting -l, -a, -F)
Pwd,
Cat,
Cp,
Rm, // (supporting -r)
Mv,
Mkdir,
Exit,
}

pub struct LsFlag{
    list_all: bool, // -a
    long_list: bool,
    list_types: bool,
}

// Enumeration to control the flags of ls and rm;
#[derive(Debug, Clone, PartialEq, Eq)]

pub enum Flag{
    L, // -l
    A, // -a
    F, // -F
    R, // -r
}

// Packaging commands togather to ease mutiple command handling:
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Commands {
     pub command: Vec<Command>,
}

// Command representer: {name: "name eg: ls, echo...", flags:vec![...flags], args:vec![...args]}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Command {
    pub name: CommandType,
    pub flags:Vec<Flag>,
    pub args: Vec<String>,
}