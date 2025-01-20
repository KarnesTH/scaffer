mod cli;
mod utils;

pub mod prelude {
    pub use crate::cli::{Cli, Commands, CreateCommand, Templates, TemplatesCommand};
    pub use crate::utils::{Config, Template};
}
