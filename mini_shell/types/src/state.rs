use std::env;
use std::ffi::NulError;
use std::path::PathBuf;
use std::cell::RefCell;
use std::fs::*;
use std::io::Error;
use env::*;

#[derive(Debug, Eq, PartialEq, Default)]
pub struct State{
pub cwd: RefCell<PathBuf>, // current working directory.
pub home: RefCell<PathBuf>, // home directory.
pub fs_entries: RefCell<Vec<PathBuf>>, // cached directory entries.
pub cmd_history: RefCell<Vec<String>>, // command history.
pub cmd_index: RefCell<Option<usize>>, // history navigation index.
}


impl State {
// initialize the state to
pub fn init_state() -> Result<State, Error> {
    let home_path = var("HOME").unwrap_or_else(|_| "/".to_string());
    let home = PathBuf::from(home_path).canonicalize()?;
    let mut fs_entries :Vec<PathBuf> = Vec::new();
    if let Ok(entries) = read_dir(&home){
        for entry in entries{
            let entry = entry?;
            fs_entries.push(entry.path());
        }
    }
    Ok(State { cwd: RefCell::new(home.clone()), home: RefCell::new(home), fs_entries: RefCell::new(fs_entries), cmd_history: RefCell::new(Vec::new()), cmd_index: RefCell::new(None)})
}

// Whenever the current directory changes the state have to be updated:
pub fn update_state(&self)-> Result<(), Error>{
    let cwd = self.cwd.borrow().clone();
    let mut new_fs_entries :Vec<PathBuf>= Vec::new();
        for entry in read_dir(&cwd)?{
            let entry = entry?;
            new_fs_entries.push(entry.path());
        }
        *self.fs_entries.borrow_mut() = new_fs_entries;
        Ok(())
    }
}
