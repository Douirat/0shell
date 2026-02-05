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
        State {
            cwd: RefCell::new(PathBuf::from(&home_path)),
            home: RefCell::new(PathBuf::from(home_path)),
        }
}
}



////////////////////////////////////////////////////////////////////////////////////
// Implémentation corrigée de l'état initial avec gestion des chemins sur Windows //
////////////////////////////////////////////////////////////////////////////////////
// use std::env;
// use std::cell::RefCell;
// use std::path::PathBuf;

// #[derive(Debug, Eq, PartialEq, Default)]
// pub struct State{
// pub cwd: RefCell<PathBuf>,
// pub home: RefCell<PathBuf>, 
// }

// impl State {
//     pub fn init_state() -> State {
//         // Utiliser le répertoire courant de l'utilisateur, pas "/"
//         let cwd = env::current_dir()
//             .unwrap_or_else(|_| {
//                 // Fallback: répertoire de l'utilisateur sur Windows
//                 if cfg!(windows) {
//                     PathBuf::from("C:\\Users\\Public")
//                 } else {
//                     PathBuf::from(".")
//                 }
//             });
        
//         // Répertoire home
//         let home = dirs::home_dir()
//             .unwrap_or_else(|| cwd.clone());
        
//         State {
//             cwd: RefCell::new(cwd),
//             home: RefCell::new(home),
//         }
//     }
// }