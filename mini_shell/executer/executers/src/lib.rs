pub mod ls; // Done by Ben,
pub mod cat; // Done by Agiel
pub mod cp; // TODO: Agiel
pub mod cd; // Done by Ben,
pub mod echo; // TODO: Agiel
pub mod exit; // Done by Agiel
pub mod mkdir; //Done by Ben,
pub mod mv; // TODO: Agiel
pub mod pwd; // Done by Ben,
pub mod rm; // Done by Ben,


pub use echo::echo;
pub use pwd::pwd;
pub use exit::exit;
pub use cd::cd;
pub use ls::ls;
pub use mkdir::mkdir;
pub use rm::rm;
pub use cat::cat;