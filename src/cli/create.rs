use std::path::PathBuf;

use inquire::{Confirm, Select, Text};

pub struct CreateCommand {
    pub language: String,
    pub name: String,
    pub path: PathBuf,
}

impl CreateCommand {
    pub fn run_create(
        &mut self,
        language: Option<String>,
        name: Option<String>,
        path: Option<PathBuf>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        println!("Creating Project...");

        let programming_languages = vec![
            "Rust",
            "Python",
            "JavaScript",
            "TypeScript",
            "Java",
            "PHP",
            "C",
            "C++",
            "C#",
        ];

        if let Some(language) = language {
            self.language = language;
        } else {
            let selected_language = Select::new(
                "Select one of these programming languages!",
                programming_languages,
            )
            .prompt()?;

            self.language = selected_language.to_string();
        }

        if let Some(name) = name {
            self.name = name;
        } else {
            let project_name = Text::new("Enter the name of the project")
                .with_help_message("This name is for your project folder name")
                .prompt()?;
            self.name = project_name;
        }

        if let Some(path) = path {
            self.path = path;
        } else {
            if !Confirm::new(
                "On default your path is the current directory. Do you want to use it?",
            )
            .prompt()?
            {
                let project_path = Text::new("Enter the path of the project")
                    .with_help_message("This path is for your project folder path")
                    .prompt()?;
                self.path = PathBuf::from(project_path);
            }
        }

        Ok(())
    }
}

impl Default for CreateCommand {
    fn default() -> Self {
        Self {
            language: String::new(),
            name: String::new(),
            path: std::env::current_dir().expect("Failed to use current directory"),
        }
    }
}
