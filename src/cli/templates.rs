use crate::utils::{Config, Template};

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
            let file_name = path.file_name().unwrap().to_str().unwrap().to_string();

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
}
