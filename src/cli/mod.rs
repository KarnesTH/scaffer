use clap::Parser;
pub use commands::Commands;
pub use create::CreateCommand;

pub mod commands;
pub mod create;

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
