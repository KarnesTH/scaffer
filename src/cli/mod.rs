use clap::Parser;
pub use commands::{Commands, TemplatesCommand};
pub use create::CreateCommand;
pub use templates::Templates;

pub mod commands;
pub mod create;
pub mod templates;

#[derive(Parser)]
#[clap(
    name = "scaffer",
    version = "0.1.0",
    about = "A simple scaffolding tool"
)]
pub struct Cli {
    #[command(subcommand)]
    pub commands: Commands,
}
