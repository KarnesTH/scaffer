use colored::*;
use std::path::{Path, PathBuf};

use inquire::{Confirm, Select, Text};

use crate::{prelude::PROGRAMMING_LANGUAGES, utils::Template};

pub struct CreateCommand {
    pub language: String,
    pub name: String,
    pub path: PathBuf,
}

impl CreateCommand {
    /// Run the create command with the given options or prompt the user for them if they are not provided
    ///
    /// # Arguments
    ///
    /// * `language` - The programming language of the project
    /// * `name` - The name of the project
    /// * `path` - The path of the project
    ///
    /// # Returns
    ///
    /// * `Result<(), Box<dyn std::error::Error>>` - The result of the create command
    ///
    /// # Errors
    ///
    /// * If the user input is invalid
    /// * If the project cannot be created
    /// * If the .gitignore file cannot be fetched
    pub fn run_create(
        &mut self,
        language: Option<String>,
        name: Option<String>,
        path: Option<PathBuf>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        println!("{}", "Creating Project...".bright_green().bold());

        let programming_languages = PROGRAMMING_LANGUAGES.to_vec();

        if let Some(language) = language {
            self.language = self.capitalize(language.as_str());
        } else {
            let selected_language = Select::new(
                "Select one of these programming languages!",
                programming_languages,
            )
            .prompt()?;

            self.language = self.capitalize(selected_language).to_string();
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
        } else if !Confirm::new("Use current directory? [y/n]").prompt()? {
            let project_path = Text::new("Enter the path of the project")
                .with_help_message("This path is for your project folder path")
                .prompt()?;
            self.path = PathBuf::from(project_path);
        }

        self.create_project()?;

        println!(
            "\n{}",
            "Project successfully created! 🎉".bright_green().bold()
        );
        println!("\n{}", "Summary:".bright_yellow().bold());
        println!("   {} Language: {}", "→".bright_blue(), self.language);
        println!("   {} Project: {}", "→".bright_blue(), self.name);
        println!(
            "   {} Location: {}",
            "→".bright_blue(),
            self.path.join(&self.name).display()
        );
        println!("\n{}", "Next steps:".bright_yellow().bold());
        println!(
            "   {} cd {}",
            "→".bright_blue(),
            self.path.join(&self.name).to_str().unwrap()
        );
        println!(
            "   {} {}",
            "→".bright_blue(),
            Template::load_template(self.language.clone())?.start_command
        );

        Ok(())
    }

    /// Create a project with the given options
    ///
    /// # Returns
    ///
    /// * `Result<(), Box<dyn std::error::Error>>` - The result of the create project
    ///
    /// # Errors
    ///
    /// * If the project cannot be created
    /// * If the .gitignore file cannot be fetched
    fn create_project(&self) -> Result<(), Box<dyn std::error::Error>> {
        let template = Template::load_template(self.language.clone())?;

        println!(
            "└─ {} {}",
            "►".bright_blue(),
            "Creating project structure...".bright_white()
        );
        let project_path = self.path.join(&self.name);
        std::fs::create_dir_all(&project_path)?;

        for folder in template.structure.directories.clone() {
            let folder_path = project_path.join(folder);
            std::fs::create_dir_all(&folder_path)?;
        }
        println!(
            "   └─ {} {}",
            "✓".bright_green(),
            "Project structure created!".green()
        );

        self.create_files(&template, &project_path)?;

        if Confirm::new("Do you want to add a .gitignore file? [y/n]").prompt()? {
            match self.fetch_gitignore(self.language.clone()) {
                Ok(_) => println!(
                    "   └─ {} {}",
                    "✓".bright_green(),
                    ".gitignore file added!".green()
                ),
                Err(_) => println!(
                    "   └─ {} {}",
                    "✗".bright_red(),
                    ".gitignore file could not be added".red()
                ),
            }
        } else {
            println!(
                "   └─ {} {}",
                "✗".bright_red(),
                ".gitignore file not added".red()
            );
        }

        Ok(())
    }

    /// Fetch the .gitignore file for the given language
    ///
    /// # Arguments
    ///
    /// * `language` - The programming language of the project
    ///
    /// # Returns
    ///
    /// * `Result<String, Box<dyn std::error::Error>>` - The result of the fetch gitignore
    fn fetch_gitignore(&self, language: String) -> Result<String, Box<dyn std::error::Error>> {
        let formatted_language = if language == "C++" {
            language.replace("+", "%2B")
        } else {
            language
        };

        let url = format!(
            "https://raw.githubusercontent.com/github/gitignore/refs/heads/main/{}.gitignore",
            formatted_language
        );

        let gitignore = ureq::get(&url).call()?.into_string()?;

        Ok(gitignore)
    }

    /// Create the files for the project
    ///
    /// # Arguments
    ///
    /// * `template` - The template of the project
    /// * `project_path` - The path of the project
    ///
    /// # Returns
    ///
    /// * `Result<(), Box<dyn std::error::Error>>` - The result of the create files
    fn create_files(
        &self,
        template: &Template,
        project_path: &Path,
    ) -> Result<(), Box<dyn std::error::Error>> {
        println!(
            "└─ {} {}",
            "►".bright_blue(),
            "Creating project files...".bright_white()
        );
        for file in template.structure.files.clone() {
            let file_path = project_path.join(file.path);
            let content = template.parse_project_name(self.name.clone(), file.content.clone())?;
            std::fs::write(&file_path, content)?;
        }
        println!(
            "   └─ {} {}",
            "✓".bright_green(),
            "Project files created!".green()
        );

        Ok(())
    }

    /// Capitalize the first letter of the given string
    ///
    /// # Arguments
    ///
    /// * `s` - The string to capitalize
    fn capitalize(&self, s: &str) -> String {
        let mut c = s.chars();
        match c.next() {
            None => String::new(),
            Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
        }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_command_default() {
        let create_command = CreateCommand::default();

        assert_eq!(create_command.language, String::new());
        assert_eq!(create_command.name, String::new());
        assert_eq!(
            create_command.path,
            std::env::current_dir().expect("Failed to use current directory")
        );
    }

    #[test]
    fn test_create_command_capitalize() {
        let create_command = CreateCommand::default();

        assert_eq!(create_command.capitalize("test"), "Test");
        assert_eq!(create_command.capitalize("Test"), "Test");
        assert_eq!(create_command.capitalize("tEST"), "TEST");
        assert_eq!(create_command.capitalize("t"), "T");
        assert_eq!(create_command.capitalize(""), "");
    }

    #[test]
    fn test_fetch_gitignore() {
        let create_command = CreateCommand::default();

        let gitignore = create_command.fetch_gitignore("Rust".to_string()).unwrap();
        assert!(gitignore.contains("target"));
    }
}
