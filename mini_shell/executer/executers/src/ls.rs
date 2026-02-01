use types::command::*;
use types::state::*;
use std::fs::*;
use std::ffi::OsString;
use std::fmt;
use std::os::unix::fs::PermissionsExt;
// use std::time::{UNIX_EPOCH, Duration}
// use chrono::{NaiveDateTime, Local};
// use std::os::unix::fs::PermissionsExt;
// use std::fs::*;
// use std::path::Path;
// use std::path::PathBuf;

// create a structure to represent the file:
#[derive(Debug, Clone, Default)]
pub struct FileEntry {
    pub permissions: Permissions,
    pub links: u64,
    pub uid: u32,
    pub gid: u32,
    pub size: u64,
    pub mtime: i64,
    pub name: OsString,
}

// represent the permission as strut as well;
#[derive(Debug, Clone, Default)]
pub struct Permissions {
    pub file_type: char, // 'd' or '-'
    pub user: [bool; 3],  // r w x
    pub group: [bool; 3],
    pub other: [bool; 3],
}

impl FileEntry{
pub fn new()-> FileEntry{
let mut entry = FileEntry::default();
entry.permissions = Permissions::default();
entry
}

pub fn init(&mut self, entry :DirEntry ){
    println!("{:?}", entry);
self.name = entry.file_name();
let meta = entry.metadata().unwrap();
let ft = meta.file_type();
let file_type = if ft.is_dir() {
    'd'
} else if ft.is_file() {
    '-'
} else if ft.is_symlink() {
    'l'
} else {
    unreachable!()
};
self.permissions.file_type = file_type;

let permissions = meta.permissions();
println!("permissios ---> {:?}", permissions.mode());



println!("{:?}", self);
}
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

    let dir = read_dir(&target)?; // Result<ReadDir> → ReadDir
/* open directory
 for each entry in directory:
     if entry is readable:
          process entry
     else:
          report error or skip
*/

    for entry in dir {
        let entry = entry?; // Result<DirEntry> → DirEntry
        let mut f = FileEntry::new();
        f.init(entry);
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