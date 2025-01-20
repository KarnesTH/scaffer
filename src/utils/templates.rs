use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use super::Config;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Template {
    pub structure: Structure,
    pub start_command: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Structure {
    pub directories: Vec<String>,
    pub files: Vec<File>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct File {
    pub path: PathBuf,
    pub content: Vec<String>,
}

impl Template {
    /// Load the template for the given language
    ///
    /// # Arguments
    ///
    /// * `language` - The programming language of the project
    ///
    /// # Returns
    ///
    /// * `Result<Self, Box<dyn std::error::Error>>` - The result of the load template
    pub fn load_template(language: String) -> Result<Self, Box<dyn std::error::Error>> {
        let config = Config::load()?;
        let template_path = config
            .template_dir
            .join(format!("{}.json", language.to_lowercase()));

        let template = std::fs::read_to_string(template_path)?;
        let template: Template = serde_json::from_str(&template)?;

        Ok(template)
    }

    /// Parse the project name in the content
    ///
    /// # Arguments
    ///
    /// * `project_name` - The name of the project
    /// * `content` - The content to parse the project name
    ///
    /// # Returns
    ///
    /// * `Result<String, Box<dyn std::error::Error>>` - The result of the parse project name
    pub fn parse_project_name(
        &self,
        project_name: String,
        content: Vec<String>,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let mut content = content.join("\n");

        content = content.replace("{{project_name}}", &project_name);

        Ok(content)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_template() {
        let template = Template::load_template("rust".to_string()).unwrap();

        assert_eq!(template.start_command, "cargo run".to_string());
        assert_eq!(template.structure.directories.len(), 1);
        assert_eq!(template.structure.files.len(), 2);
    }

    #[test]
    fn test_parse_project_name() {
        let template = Template::load_template("rust".to_string()).unwrap();
        let content = vec!["{{project_name}}".to_string()];

        let parsed_content = template
            .parse_project_name("test".to_string(), content)
            .unwrap();

        assert_eq!(parsed_content, "test".to_string());
    }
}
