use types::command::*;
use types::state::*;
// use std::os::unix::fs::PermissionsExt;
use std::fs::*;
// use std::path::Path;
// use std::path::PathBuf;

// create a structure to represent the file:
#[derive(Debug, Clone)]
pub struct FileEntry {
    pub permissions: Permissions,
    pub links: u64,
    pub owner: String,
    pub group: String,
    pub size: u64,
    pub month: String,
    pub day: u8,
    pub time: String,
    pub name: String,
}

// represent the permission as strut as well;
#[derive(Debug, Clone)]
pub struct Permissions {
    pub file_type: char, // 'd' or '-'
    pub user: [bool; 3],  // r w x
    pub group: [bool; 3],
    pub other: [bool; 3],
}



pub fn ls<'a>(command: &Command) {
// let marker = 0;
if command.args.is_empty() {
   let _ = list(&command.state.clone(), ".".to_string());
} else {
    for arg in &command.args{
        let _ = list(&command.state.clone(), arg.clone());
    }
}
}

  fn list(state: &State, arg: String) -> Result<(), Box<dyn std::error::Error>> {
    let target = state.cwd.borrow().join(arg).canonicalize()?;

    let dir = std::fs::read_dir(&target)?; // Result<ReadDir> → ReadDir

    for entry in dir {
        let entry = entry?; // Result<DirEntry> → DirEntry
        println!("{:?}", entry.file_name());
    }

    Ok(())
}

// #[derive(Debug, Clone, PartialEq, Eq)]
// pub struct Command<'a> {
//     pub name: CommandType,
//     pub flags:Vec<Flag>,
//     pub args: Vec<String>,
//     pub state: &'a Rc<State>,
// }