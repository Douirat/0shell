// A command enumeration to limit the falling input and redirect the distinction:
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CommandType{
echo,
cd,
ls, //(supporting -l, -a, -F)
pwd,
cat,
cp,
rm, // (supporting -r)
mv,
mkdir,
exit,
}

// Enumeration to control the flags of ls and rm;
pub enum Flag{
    l,
    a,
    f,
    r,
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
    pub args: Vec<String>,
}


