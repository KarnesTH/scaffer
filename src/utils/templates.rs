use std::path::PathBuf;

use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Template {
    pub structure: Structure,
}

#[derive(Deserialize, Debug)]
pub struct Structure {
    pub directories: Vec<String>,
    pub files: Vec<File>,
}

#[derive(Deserialize, Debug)]
pub struct File {
    pub path: PathBuf,
    pub content: Vec<String>,
}

impl Template {
    pub fn load_template(language: String) -> Result<Self, Box<dyn std::error::Error>> {
        let template_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("templates")
            .join(format!("{}.json", language.to_lowercase()));

        let template = std::fs::read_to_string(template_path)?;
        let template: Template = serde_json::from_str(&template)?;

        Ok(template)
    }
}
