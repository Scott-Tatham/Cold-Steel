/*!
 * Defines the types of suite events that are rendered.
 */
use serde::Deserialize;
use super::super::result::SuiteResultEvent;
use super::super::started::SuiteStartedEvent;

/// Defines the types of suite events that are rendered.
#[derive(Debug, Clone, Copy, PartialEq, Deserialize)]
#[serde(tag = "event", rename_all = "lowercase")]
pub enum SuiteEvent
{
    /// The resulting 'started' event for a test suite.
    Started(SuiteStartedEvent),

    /// The resulting 'ok' event for a test suite.
    Ok(SuiteResultEvent),

    /// The resulting 'failed' event for a test suite.
    Failed(SuiteResultEvent)
}