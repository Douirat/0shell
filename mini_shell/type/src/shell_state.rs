use std::path::PathBuf;

#[derive(Debug, Eq, PartialEq)]
pub struct State{
pub cwd: PathBuf, //TODO: remember adding cellRef to all changing even when immutable access is provided:
pub home: Option<PathBuf>, 
}