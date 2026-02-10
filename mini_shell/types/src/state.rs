use std::env;
use std::path::PathBuf;
use std::cell::RefCell;

#[derive(Debug, Eq, PartialEq, Default)]
pub struct State{
pub cwd: RefCell<PathBuf>,
pub home: RefCell<PathBuf>, 
}


impl State {
// initialize the state to
pub fn init_state() -> State {
      let home_path = env::var("HOME").unwrap_or_else(|_| "/".to_string());
      let cwd_path = env::current_dir().unwrap_or_else(|_| PathBuf::from(&home_path));
        State {
            cwd: RefCell::new(cwd_path),
            home: RefCell::new(PathBuf::from(home_path)),
        }
}
}