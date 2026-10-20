/*!
 * Handles the rendering of text.
 */
use std::env;
use std::sync::OnceLock;
use supports_unicode::Stream;

/// Cached Nerd Font detection result.
static NERD_FONT_DETECTED: OnceLock<bool> = OnceLock::new();

/// Cached Unicode support detection result.
static UNICODE_SUPPORTED: OnceLock<bool> = OnceLock::new();

/// Determines whether a Nerd Font is detected.
/// # Returns
/// `bool` - Returns whether a Nerd Font is detected.
fn is_nerd_font_detected() -> bool
{
    *NERD_FONT_DETECTED.get_or_init(| |
    {
        let environment_variables: Vec<(String, String)> = env::vars().collect();
        has_nerd_font::detect(&environment_variables).detected == Some(true)
    })
}

/// Determines whether a Unicode is supported.
/// # Returns
/// `bool` - Returns whether a Nerd Font is supported.
fn is_unicode_supported() -> bool
{
    *UNICODE_SUPPORTED.get_or_init(| |
    {
        supports_unicode::on(Stream::Stdout)
    })
}

/// Renders text based on the environment.
/// # Arguments
/// * `nerd_font_text` - The text to render if a Nerd Font is detected.
/// * `unicode_text` - The text to render if Unicode is supported.
/// * `ascii_text` - The text to render as a fallback.
/// # Returns
/// `String` - The rendered text.
pub fn render_text(nerd_font_text: &str, unicode_text: &str, ascii_text: &str) -> String
{
    if is_nerd_font_detected()
    {
        return nerd_font_text.to_string();
    }

    if is_unicode_supported()
    {
        return unicode_text.to_string();
    }

    ascii_text.to_string()
}