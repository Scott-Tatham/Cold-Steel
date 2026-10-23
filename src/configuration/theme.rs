/*!
 * Stores a theme for Cold Steel.
 */
use std::str::FromStr;
use serde::{Serialize, Deserialize, Serializer, Deserializer};
use serde::de::Error;
use ratatui::style::Color;

/// Stores a theme for Cold Steel.
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Theme
{
    /// The background colour of the theme.
    #[serde(deserialize_with = "deserialise_colour", serialize_with = "serialise_color")]
    pub background: Color,

    /// The text colour of the theme.
    #[serde(deserialize_with = "deserialise_colour", serialize_with = "serialise_color")]
    pub text: Color,
    
    /// The title colour of the theme.
    #[serde(deserialize_with = "deserialise_colour", serialize_with = "serialise_color")]
    pub title: Color,

    /// The passed colour of the theme.
    #[serde(deserialize_with = "deserialise_colour", serialize_with = "serialise_color")]
    pub passed: Color,

    /// The failed colour of the theme.
    #[serde(deserialize_with = "deserialise_colour", serialize_with = "serialise_color")]
    pub failed: Color,

    /// The highlight colour of the theme.
    #[serde(deserialize_with = "deserialise_colour", serialize_with = "serialise_color")]
    pub highlight: Color,

    /// The border colour of the theme.
    #[serde(deserialize_with = "deserialise_colour", serialize_with = "serialise_color")]
    pub border: Color
}

/// The definition of the [Theme].
impl Theme
{
    /// Initialises a new instance of the [Theme] struct.
    /// # Arguments
    /// * `background` - The background colour of the theme.
    /// * `text` - The text colour of the theme.
    /// * `title` - The title colour of the theme.
    /// * `passed` - The passed colour of the theme.
    /// * `failed` - The failed colour of the theme.
    /// * `highlight` - The highlight colour of the theme.
    /// * `border` - The border colour of the theme.
    /// # Returns
    /// `Theme` - Returns the new instance of the [Theme] struct.
    pub fn from_colours(background: Color, text: Color, title: Color, passed: Color, failed: Color, highlight: Color, border: Color) -> Theme
    {
        Theme
        {
            background,
            text,
            title,
            passed,
            failed,
            highlight,
            border
        }
    }

    /// Initialises a new instance of the [Theme] struct.
    /// # Arguments
    /// * `background` - The background colour of the theme.
    /// * `text` - The text colour of the theme.
    /// * `title` - The title colour of the theme.
    /// * `passed` - The passed colour of the theme.
    /// * `failed` - The failed colour of the theme.
    /// * `highlight` - The highlight colour of the theme.
    /// * `border` - The border colour of the theme.
    /// # Returns
    /// `Theme` - Returns the new instance of the [Theme] struct.
    pub fn from_strings(background: String, text: String, title: String, passed: String, failed: String, highlight: String, border: String) -> Theme
    {
        Theme
        {
            background: Color::from_str(&background).unwrap_or(Color::Reset),
            text: Color::from_str(&text).unwrap_or(Color::Reset),
            title: Color::from_str(&title).unwrap_or(Color::Reset),
            passed: Color::from_str(&passed).unwrap_or(Color::Reset),
            failed: Color::from_str(&failed).unwrap_or(Color::Reset),
            highlight: Color::from_str(&highlight).unwrap_or(Color::Reset),
            border: Color::from_str(&border).unwrap_or(Color::Reset)
        }
    }
}

/// Serialises a string into colour.
/// # Arguments
/// * `colour` - The colour to serialise.
/// * `serialiser` - The serialiser to use.
/// # Returns
/// `Result<S::Ok, S::Error>` - Returns the result of the serialisation or an error.
/// # Errors
/// Returns an error if the colour cannot be serialised.
fn serialise_color<S>(colour: &Color, serialiser: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serialiser.collect_str(colour)
}

/// Deserialises a colour from a string.
/// # Arguments
/// * `deserialiser` - The deserialiser to use.
/// # Returns
/// `Result<Color, D::Error>` - Returns the result containing the deserialised colour or an error.
/// # Errors
/// Returns an error if the colour cannot be deserialised.
fn deserialise_colour<'a, D>(deserialiser: D) -> Result<Color, D::Error>
where
    D: Deserializer<'a>
{
    let string = String::deserialize(deserialiser)?;
    Color::from_str(&string).map_err(Error::custom)
}