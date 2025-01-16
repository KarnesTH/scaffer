use std::path::PathBuf;

use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct Template {
    pub structure: Structure,
    pub start_command: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Structure {
    pub directories: Vec<String>,
    pub files: Vec<File>,
}

#[derive(Deserialize, Debug, Clone)]
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
        let template_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("templates")
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
