/*!
 * Stores the result event for a test case.
 */
use serde::Deserialize;

/// Stores the result event for a test case.
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct TestResultEvent
{
    /// The name of the test case.
    pub name: String,

    /// The time taken to execute the test case.
    pub exec_time: Option<f64>,

    /// The failure message of the test case.
    pub stdout: Option<String>
}

/// Tests for the [TestResultEvent].
#[cfg(test)]
mod tests
{
    use serde_json::json;
    use super::super::enums::test_event::TestEvent;
    use super::super::super::enums::event::Event;

    /// Tests the parsing of a test result event.
    #[test]
    fn test_parse_test_result_event()
    {
        let json = json!(
        {
            "type": "test",
            "event": "ok",
            "name": "test_test",
            "exec_time": 1.0,
            "stdout": "test_stdout"
        });

        let event: Event = serde_json::from_value(json).unwrap();
        assert!(matches!(event, Event::Test(TestEvent::Ok(_))))
    }
}