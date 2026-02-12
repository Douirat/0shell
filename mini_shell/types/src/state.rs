use std::env;
use std::path::PathBuf;
use std::cell::RefCell;

#[derive(Debug, Eq, PartialEq, Default)]
pub struct State{
pub user: String, // constant.
pub host: String, // constant.
pub home: PathBuf, // constant.
pub cwd: RefCell<PathBuf>, // RefCell cause it going to issue an internal change.
}


impl State {
// initialize the state to
pub fn init_state() -> State {
      let home_path = env::var("HOME").unwrap_or_else(|_| "/".to_string());
        State {
            user: env::var("USER").unwrap_or_else(|_| "unknown".to_string()),
            host: hostname::get()
                .unwrap_or_default()
                .to_string_lossy()
                .into_owned(),
            home: PathBuf::from(&home_path),
            cwd: RefCell::new(PathBuf::from(home_path)),
        }
}
}