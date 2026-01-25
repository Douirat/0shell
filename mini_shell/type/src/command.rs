// A command enumeration to limit the falling input and redirect the distinction:
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CommandType{
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
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct LsFlag{
    pub list_all: bool, // -a
    pub long_list: bool, // -l
    pub list_types: bool, // -F
}

// Enumeration to control the flags of ls and rm;
#[derive(Debug, Clone, PartialEq, Eq, )]
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
pub struct Command<'a> {
    pub name: CommandType,
    pub flags:Vec<Flag>,
    pub args: Vec<String>,
    pub state: &'a State,
}