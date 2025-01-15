mod cli;
mod utils;

pub mod prelude {
    pub use crate::cli::{Cli, Commands, CreateCommand};
    pub use crate::utils::Template;
}
