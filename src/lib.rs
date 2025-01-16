mod cli;
mod utils;

pub mod prelude {
    pub use crate::cli::{Cli, Commands, CreateCommand};
    pub use crate::utils::Template;

    pub const PROGRAMMING_LANGUAGES: [&str; 8] =
        ["Rust", "Python", "Java", "PHP", "C", "C++", "HTML", "Go"];
}
