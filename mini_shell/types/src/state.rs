use std::env;
use std::path::PathBuf;
use std::cell::RefCell;

#[derive(Debug, Eq, PartialEq, Default)]
pub struct State{
pub cwd: RefCell<PathBuf>,
pub home: RefCell<PathBuf>,
pub fs_entries: RefCell<Vec<PathBuf>>,
pub cmd_history: RefCell<Vec<String>>,
pub cmd_index: RefCell<Option<usize>>,
}


impl State {
// initialize the state to
pub fn init_state() -> State {
      let home_path = env::var("HOME").unwrap_or_else(|_| "/".to_string());
        State {
            cwd: RefCell::new(PathBuf::from(&home_path)),
            home: RefCell::new(PathBuf::from(&home_path)),
        }
}
}