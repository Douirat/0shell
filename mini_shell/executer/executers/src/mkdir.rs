use types::command::*;

pub fn mkdir(command: &Command){
    for arg in &command.args{
        let path = command.state.cwd.borrow().join(&arg);
            if let Err(e) = std::fs::create_dir(&path) {
            eprintln!("mkdir: {}: {}", path.to_string_lossy(), e);
    }
    }
}