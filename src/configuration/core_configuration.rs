/*!
 * Stores the configuration for Cold Steel.
 */
use indexmap::IndexMap;
use std::fs;
use std::path::PathBuf;
use directories::ProjectDirs;
use serde::{Serialize, Deserialize};
use super::theme::Theme;

/// Stores the configuration for Cold Steel.
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Configuration
{
    /// The current theme.
    pub theme: Option<String>,

    /// The collection of possible themes.
    pub themes: IndexMap<String, Theme>
}

/// The implementation of the [Configuration] struct.
impl Configuration
{
    /// Initialises a new, default instance of the [Configuration] struct.
    /// # Returns
    /// `Configuration` - Returns the new, default instance of the [Configuration] struct.
    pub fn default() -> Self
    {
        Configuration
        {
            theme: Some("default".to_string()),
            themes: IndexMap::from(
                [
                    ("default".to_string(), Theme::from_strings("#FBE1B5".to_string(), "#2E434F".to_string(), "#759DA9".to_string(), "#8DA249".to_string(), "#F89254".to_string(), "#759DA9".to_string(), "#2E434F".to_string())),
                    ("light".to_string(), Theme::from_strings("#FBE1B5".to_string(), "#2E434F".to_string(), "#759DA9".to_string(), "#8DA249".to_string(), "#F89254".to_string(), "#759DA9".to_string(), "#2E434F".to_string())),
                    ("dark".to_string(), Theme::from_strings("#2E434F".to_string(), "#FBE1B5".to_string(), "#759DA9".to_string(), "#8DA249".to_string(), "#F89254".to_string(), "#759DA9".to_string(), "#FBE1B5".to_string()))
                ]
            )
        }
    }
}

/// Gets the path to the configuration file.
/// # Returns
/// `PathBuf` - Returns the path to the configuration file.
fn configuration_path() -> PathBuf
{
    if let Some(project_directories) = ProjectDirs::from("dev", "chicken-lips", "cold-steel")
    {
        project_directories.config_dir().join("configuration.toml")
    }

    else
    {
        dirs::home_dir().unwrap().join("configuration.yaml")
    }
}

/// Loads the configuration or creates a new one when the configuration file does not exist.
/// # Returns
/// `Configuration` - Returns the loaded or created configuration.
pub fn load_or_create_configuration() -> Configuration
{
    let configuration_path = configuration_path();

    match fs::read_to_string(&configuration_path)
    {
        Ok(content) => toml::from_str(&content).expect("Failed to parse Cold Steel configuration file."),
        Err(_) =>
            {
                let base_configuration = Configuration::default();
                let toml_value = toml::to_string(&base_configuration).unwrap();

                if let Some(parent) = configuration_path.parent()
                {
                    fs::create_dir_all(parent).expect("Failed to create the configuration directory.");
                }

                fs::write(&configuration_path, toml_value).expect("Failed to write the default configuration file.");

                base_configuration
            }
    }
}

/**
 * Tests for the core configuration file.
 */
#[cfg(test)]
mod tests
{
    use super::configuration_path;

    /**
     * Tests the path of the configuration file is not empty.
     */
    #[test]
    fn test_configuration_path_is_not_empty()
    {
        assert!(!configuration_path().as_os_str().is_empty());
    }
}