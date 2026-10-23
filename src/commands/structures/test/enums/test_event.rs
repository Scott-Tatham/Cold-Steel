/*!
 * Stores the test event.
 */
use serde::Deserialize;
use super::super::result::TestResultEvent;
use super::super::started::TestStartedEvent;

/// Stores the test event.
#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(tag = "event", rename_all = "snake_case")]
pub enum TestEvent
{
    /// The resulting 'started' event for a test case.
    Started(TestStartedEvent),

    /// The resulting 'ok' event for a test case.
    Ok(TestResultEvent),

    /// The resulting 'failed' event for a test case.
    Failed(TestResultEvent),

    /// The resulting 'ignored' event for a test case.
    Ignored(TestResultEvent)
}