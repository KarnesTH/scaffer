use std::path::PathBuf;

use clap::Subcommand;

#[derive(Subcommand)]
pub enum Commands {
    /// Create a new project from a template
    Create {
        /// The programming language of the project
        #[arg(short, long)]
        language: Option<String>,
        /// The name of the project
        #[arg(short, long)]
        name: Option<String>,
        /// The installation path of the project
        #[arg(short, long)]
        path: Option<PathBuf>,
    },
    /// Manage the available templates
    Templates {
        #[command(subcommand)]
        subcommand: TemplatesCommand,
    },
}

#[derive(Subcommand)]
pub enum TemplatesCommand {
    /// List the available templates
    List {
        /// The programming language to filter the templates
        #[arg(short, long)]
        filter: Option<String>,
    },
    /// Add a new template
    Add {
        /// The programming language of the template
        #[arg(short, long)]
        language: Option<String>,
    },
    /// Remove a template
    Remove {
        /// The programming language of the template
        #[arg(short, long)]
        template: Option<String>,
    },
    /// Update a template
    Update {
        /// The programming language of the template
        #[arg(short, long)]
        language: Option<String>,
    },
}
