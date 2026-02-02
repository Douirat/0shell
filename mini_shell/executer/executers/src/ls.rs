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
    pub permissions: String,
    pub links: u64,
    pub uid: u32,
    pub gid: u32,
    pub size: u64,
    pub mtime: i64,
    pub name: OsString,
}


impl FileEntry{
pub fn new()-> FileEntry{
 FileEntry::default()
}

pub fn init(&mut self, entry :DirEntry ){
self.name = entry.file_name();
let meta = entry.metadata().unwrap();
  println!("{:?}", meta);
let ft = &meta.file_type();
let file_type = match (ft.is_file(), ft.is_dir(), ft.is_symlink()){
    (true, false, false) => 'd',
    (false, true, false) => '-',
    (false, false, true) => 'l',
    (_, _, _) => unreachable!(),
};


let permissions = meta.permissions().mode();
let perm_string = perms_to_string(permissions);
self.permissions = file_type.to_string() + &perm_string;




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
fn perms_to_string(mode: u32) -> String {
    let mut s = String::new();

    let flags = [
        (0o400, 'r'),
        (0o200, 'w'),
        (0o100, 'x'),
        (0o040, 'r'),
        (0o020, 'w'),
        (0o010, 'x'),
        (0o004, 'r'),
        (0o002, 'w'),
        (0o001, 'x'),
    ];

    for (bit, ch) in flags {
        if mode & bit != 0 {
            s.push(ch);
        } else {
            s.push('-');
        }
    }

    s
}
