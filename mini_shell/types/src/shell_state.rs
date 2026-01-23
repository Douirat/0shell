use std::path::PathBuf;

#[derive(Debug, Eq, PartialmEq)]
pub struct State{
current_dir: PathBuf, //TODO: remember adding cellRef to all changing even when immutable access is provided:
previous_dir: Option<PathBuf>, 
}