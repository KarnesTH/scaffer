use std::{collections::HashMap, path::PathBuf};

use inquire::{
    ui::{Color, RenderConfig, Styled},
    Confirm, Editor, Select, Text,
};

use crate::utils::{Config, File, Structure, Template};

pub struct Templates {
    pub templates: Vec<Template>,
}

impl Templates {
    /// List the available templates
    ///
    /// # Arguments
    ///
    /// * `filter` - The filter to apply to the templates
    /// * `config` - The configuration object
    ///
    /// # Returns
    ///
    /// The list of available templates
    ///
    /// # Errors
    ///
    /// This function will return an error if the templates cannot be listed
    pub fn list_templates(
        filter: Option<String>,
        config: &Config,
    ) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let mut templates = vec![];

        let template_dir = config.template_dir.clone();

        for entry in std::fs::read_dir(template_dir)? {
            let entry = entry?;
            let path = entry.path();
            let file_name = path
                .file_name()
                .unwrap()
                .to_str()
                .unwrap()
                .to_string()
                .replace(".json", "");

            if let Some(filter) = &filter {
                if file_name.contains(filter) {
                    templates.push(file_name);
                }
            } else {
                templates.push(file_name);
            }
        }

        Ok(templates)
    }

    /// Add a new template
    ///
    /// # Arguments
    ///
    /// * `language` - The language of the template
    ///
    /// # Returns
    ///
    /// The result of adding a new template
    ///
    /// # Errors
    ///
    /// This function will return an error if the template cannot be added
    pub fn add_template(language: Option<String>) -> Result<(), Box<dyn std::error::Error>> {
        let language = if let Some(language) = language {
            language
        } else {
            Text::new("Enter the language of the template. (e.g. python, rust etc)").prompt()?
        };

        let directories = Self::parse_directories()?;

        let files = Self::parse_files()?;

        let structure = Structure {
            directories,
            files: files
                .iter()
                .map(|(path, content)| File {
                    path: PathBuf::from(path),
                    content: vec![content.clone()],
                })
                .collect(),
        };

        let start_command = Text::new("Please enter the start command:")
            .with_help_message(
                "Enter the start command for the project. (e.g. cargo run, python main.py)",
            )
            .prompt()?;

        let template = Template {
            structure,
            start_command,
        };

        let config = Config::load()?;
        let template_path = config
            .template_dir
            .join(format!("{}.json", language.to_lowercase()));

        let template = serde_json::to_string(&template)?;
        std::fs::write(template_path, template)?;

        println!("Successfully added new template");

        Ok(())
    }

    /// Remove a template
    ///
    /// # Arguments
    ///
    /// * `template` - The template to remove
    ///
    /// # Returns
    ///
    /// The result of removing a template
    ///
    /// # Errors
    ///
    /// This function will return an error if the template cannot be removed
    pub fn remove_template(template: Option<String>) -> Result<(), Box<dyn std::error::Error>> {
        let config = Config::load()?;

        let template_list = Self::list_templates(template.clone(), &config)?;

        let template_path = if let Some(template) = template {
            config
                .template_dir
                .join(format!("{}.json", template.to_lowercase()))
        } else {
            let selelected_template =
                Select::new("Please select the template to remove:", template_list).prompt()?;
            config
                .template_dir
                .join(format!("{}.json", selelected_template.to_lowercase()))
        };

        if template_path.exists() {
            std::fs::remove_file(template_path)?;
            println!("Successfully removed template");
        } else {
            println!("Template does not exist");
        }

        Ok(())
    }

    /// Parse the directories
    ///
    /// Take the input from the user to add the directories information
    ///
    /// # Returns
    ///
    /// The list of directories
    ///
    /// # Errors
    ///
    /// This function will return an error if the directories cannot be parsed
    fn parse_directories() -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let mut directories = vec![];

        let add_dictionary = Text::new("Please enter your dictonaries:")
            .with_help_message("Enter your directonaries comma sperated. (e.g src, assets)")
            .prompt()?;

        for directory in add_dictionary.split(",") {
            directories.push(directory.to_string());
        }

        Ok(directories)
    }

    /// Parse the files
    ///
    /// Take the input from the user to add the files information
    ///
    /// # Returns
    ///
    /// The list of files
    ///
    /// # Errors
    ///
    /// This function will return an error if the files cannot be parsed
    fn parse_files() -> Result<HashMap<String, String>, Box<dyn std::error::Error>> {
        let mut files = HashMap::new();

        loop {
            let add_file = Confirm::new("Do you want to add a file?").prompt()?;

            if !add_file {
                break;
            }

            let path = Text::new("Please enter the path your file name:")
                .with_help_message("Enter your file path. (e.g main.py, index.html, src/main.rs)")
                .prompt()?;

            let content = Editor::new("Please enter your content:")
                .with_formatter(&|submission| {
                    let char_count = submission.chars().count();
                    if char_count == 0 {
                        String::from("<skipped>")
                    } else if char_count <= 20 {
                        submission.into()
                    } else {
                        let mut substr: String = submission.chars().take(17).collect();
                        substr.push_str("...");
                        substr
                    }
                })
                .with_render_config(Self::content_render_config())
                .prompt()?;

            files.insert(path, content);
        }

        Ok(files)
    }

    /// The render configuration for the content
    ///
    /// # Returns
    ///
    /// The render configuration for the content
    ///
    /// # Errors
    ///
    /// This function will return an error if the render configuration cannot be created
    fn content_render_config() -> RenderConfig<'static> {
        RenderConfig::default()
            .with_canceled_prompt_indicator(Styled::new("<skipped>").with_fg(Color::DarkYellow))
    }
}
