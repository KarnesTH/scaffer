use std::path::PathBuf;

use clap::Subcommand;

#[derive(Subcommand)]
pub enum Commands {
    /// Create a new project from a template
    Create {
        /// The programming language of the project
        language: Option<String>,
        /// The name of the project
        name: Option<String>,
        /// The installation path of the project
        path: Option<PathBuf>,
    },
}
