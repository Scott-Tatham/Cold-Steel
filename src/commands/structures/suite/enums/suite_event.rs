/*!
 * Stores the suite event.
 */
use serde::Deserialize;
use super::super::result::SuiteResultEvent;
use super::super::started::SuiteStartedEvent;

/// Stores the suite event.
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