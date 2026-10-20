/*!
 * Stores the 'started' event for a test suite.
 */
use serde::Deserialize;

/// Stores the 'started' event for a test suite.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
pub struct SuiteStartedEvent
{
    /// The number of tests in the suite.
    pub test_count: Option<u64>
}

/// Tests for the [SuiteStartedEvent].
#[cfg(test)]
mod tests
{
    use serde_json::json;
    use super::super::enums::suite_event::SuiteEvent;
    use super::super::super::enums::event::Event;

    /// Tests the parsing of a suite started event.
    #[test]
    fn test_parse_suite_started_event()
    {
        let json = json!(
        {
            "type": "suite",
            "event": "started",
            "test_count": 1
        });

        let event: Event = serde_json::from_value(json).unwrap();
        assert!(matches!(event, Event::Suite(SuiteEvent::Started(_))))
    }
}