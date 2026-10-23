/*!
 * Stores the configuration for Cold Steel.
 */
use indexmap::IndexMap;
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

/// The definition of the [Configuration].
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