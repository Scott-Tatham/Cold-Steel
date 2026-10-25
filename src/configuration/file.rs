use std::fs;
use std::path::PathBuf;
use directories::ProjectDirs;
use super::core_configuration::Configuration;

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
        dirs::home_dir().expect("Failed to get the home directory.").join("configuration.yaml")
    }
}

/// Loads the configuration or creates a new one when the configuration file does not exist.
/// # Returns
/// `Configuration` - Returns the loaded or created configuration.
pub fn load_or_create_configuration() -> Configuration
{
    let configuration_path = configuration_path();
    let read_result = fs::read_to_string(&configuration_path);
    
    match read_result
    {
        Ok(content) => toml::from_str(&content).expect("Failed to parse Cold Steel configuration file."),
        Err(_) =>
            {
                let base_configuration = Configuration::default();
                let toml_value = toml::to_string(&base_configuration).expect("Failed to serialise configuration.");

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