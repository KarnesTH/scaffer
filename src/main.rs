use clap::Parser;
use scaffer::prelude::*;

/// The main entry point of the application
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::load()?;

    let cli = Cli::parse();

    match cli.commands {
        Commands::Create {
            language,
            name,
            path,
        } => {
            let mut create_command = CreateCommand::default();
            create_command.run_create(language, name, path, &config)?;
        }
        Commands::Templates { subcommand } => match subcommand {
            TemplatesCommand::List { filter } => {
                let templates = Templates::list_templates(filter, &config)?;
                println!("Avaiable templates:");
                for template in templates {
                    println!("- {}", template);
                }
            }
            TemplatesCommand::Add { language } => {
                if let Some(language) = language {
                    println!("Adding with language: {}", language);
                }

                println!("Adding template...");
            }
        },
    }

    Ok(())
}
