use types::State;
use std::env;
use std::path::PathBuf;


impl State {
    // initialize the state to
pub fn init_state() -> State {
    let mut state = State::default();

    if let Ok(home) = env::var("HOME") {
        state.cwd = PathBuf::from(&home);
        state.home = PathBuf::from(home);
    }

    state
}

}