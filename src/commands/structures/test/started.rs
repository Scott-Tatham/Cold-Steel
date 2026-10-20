/*!
 * Stores the 'started' event for a test case.
 */
use serde::Deserialize;

/// Stores the 'started' event for a test case.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct TestStartedEvent
{
    /// The name of the test case.
    pub name: String
}

/// Tests for the [TestStartedEvent].
#[cfg(test)]
mod tests
{
    use serde_json::json;
    use super::super::enums::test_event::TestEvent;
    use super::super::super::enums::event::Event;

    /// Tests the parsing of a test started event.
    #[test]
    fn test_parse_test_result_event()
    {
        let json = json!(
        {
            "type": "test",
            "event": "started",
            "name": "test_test"
        });

        let event: Event = serde_json::from_value(json).unwrap();
        assert!(matches!(event, Event::Test(TestEvent::Started(_))))
    }
}