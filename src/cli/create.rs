use std::path::PathBuf;

use inquire::{Confirm, Select, Text};

use crate::utils::Template;

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
            "HTML",
            "Go",
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
        } else if !Confirm::new(
            "On default your path is the current directory. Do you want to use it?",
        )
        .prompt()?
        {
            let project_path = Text::new("Enter the path of the project")
                .with_help_message("This path is for your project folder path")
                .prompt()?;
            self.path = PathBuf::from(project_path);
        }

        self.create_project()?;

        Ok(())
    }

    fn create_project(&self) -> Result<(), Box<dyn std::error::Error>> {
        let template = Template::load_template(self.language.clone())?;

        let project_path = self.path.join(&self.name);
        std::fs::create_dir_all(&project_path)?;

        for folder in template.structure.directories.clone() {
            let folder_path = project_path.join(folder);
            std::fs::create_dir_all(&folder_path)?;
        }

        self.create_files(&template, &project_path)?;

        if Confirm::new("Do you want to add a .gitignore file?").prompt()? {
            if let Ok(gitignore) = self.fetch_gitignore(self.language.clone()) {
                let gitignore_path = project_path.join(".gitignore");
                std::fs::write(&gitignore_path, gitignore)?;
            } else {
                println!("Couldn't fetch .gitignore file for this language");
            }
        }

        Ok(())
    }

    fn fetch_gitignore(&self, language: String) -> Result<String, Box<dyn std::error::Error>> {
        let url = format!(
            "https://raw.githubusercontent.com/github/gitignore/refs/heads/main/{}.gitignore",
            language
        );

        let gitignore = ureq::get(&url).call()?.into_string()?;

        Ok(gitignore)
    }

    fn create_files(
        &self,
        template: &Template,
        project_path: &PathBuf,
    ) -> Result<(), Box<dyn std::error::Error>> {
        for file in template.structure.files.clone() {
            let file_path = project_path.join(file.path);
            let content = template.parse_project_name(self.name.clone(), file.content.clone())?;
            std::fs::write(&file_path, content)?;
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
