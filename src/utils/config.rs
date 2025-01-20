use std::path::PathBuf;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    pub template_dir: PathBuf,
    pub languages: Vec<String>,
    pub theme: String,
}

impl Config {
    /// Load the configuration from the config file
    ///
    /// # Returns
    ///
    /// The configuration object
    ///
    /// # Errors
    ///
    /// This function will return an error if the config file does not exist or if the config file
    pub fn load() -> Result<Self, Box<dyn std::error::Error>> {
        let config_dir = Self::get_config_dir()?;
        let config_file = config_dir.join("config.toml");

        if !config_file.exists() {
            Self::init()?;
        }

        let config = std::fs::read_to_string(config_file)?;
        let config: Config = toml::from_str(&config)?;

        Ok(config)
    }

    /// Get the configuration directory
    ///
    /// # Returns
    ///
    /// The configuration directory
    ///
    /// # Errors
    ///
    /// This function will return an error if the configuration directory does not exist and cannot be created
    fn get_config_dir() -> Result<PathBuf, Box<dyn std::error::Error>> {
        let config = dirs::config_dir().ok_or("Could not find config directory")?;
        let config_dir = config.join("karnes-development/scaffer");
        if !config_dir.exists() {
            std::fs::create_dir_all(&config_dir)?;
        }
        Ok(config_dir)
    }

    /// Save the configuration to the config file
    ///
    /// # Errors
    ///
    /// This function will return an error if the configuration cannot be saved
    pub fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        let config_dir = Self::get_config_dir()?;
        let config_file = config_dir.join("config.toml");
        let config = toml::to_string(self)?;
        std::fs::write(config_file, config)?;
        Ok(())
    }

    /// Initialize the configuration
    ///
    /// # Errors
    ///
    /// This function will return an error if the configuration cannot be initialized
    pub fn init() -> Result<(), Box<dyn std::error::Error>> {
        let config_dir = Self::get_config_dir()?;
        let config_file = config_dir.join("config.toml");
        let template_dir = config_dir.join("templates");

        if !template_dir.exists() {
            std::fs::create_dir(&template_dir)?;
        }

        let default_languages = vec![
            "Rust".to_string(),
            "Python".to_string(),
            "Java".to_string(),
            "PHP".to_string(),
            "C".to_string(),
            "C++".to_string(),
            "HTML".to_string(),
            "Go".to_string(),
        ];

        let config = Config {
            template_dir: template_dir.clone(),
            languages: default_languages,
            theme: "default".to_string(),
        };

        let config = toml::to_string(&config)?;
        std::fs::write(config_file, config)?;

        let default_templates_path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("templates");

        for entry in std::fs::read_dir(default_templates_path)? {
            let entry = entry?;
            let path = entry.path();
            let file_name = path.file_name().unwrap().to_str().unwrap();
            let dest = template_dir.join(file_name);
            std::fs::copy(path, dest)?;
        }

        Ok(())
    }
}
