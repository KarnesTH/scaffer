use clap::Parser;
use scaffer::prelude::*;

/// The main entry point of the application
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match cli.commands {
        Commands::Create {
            language,
            name,
            path,
        } => {
            let mut create_command = CreateCommand::default();
            create_command.run_create(language, name, path)?;
        }
    }

    Ok(())
}
