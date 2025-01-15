use clap::Parser;
pub use commands::Commands;

pub mod commands;

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
