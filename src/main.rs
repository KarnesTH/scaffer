use clap::Parser;
use scaffer::prelude::*;

fn main() {
    let cli = Cli::parse();

    match cli.commands {
        Commands::Create {
            language,
            name,
            path,
        } => {
            println!("Creating a new project from a template");
            println!("Language: {:?}", language);
            println!("Name: {:?}", name);
            println!("Path: {:?}", path);
        }
    }
}
