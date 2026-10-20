/*!
 * Stores the result for a test suite.
 */
use serde::Deserialize;

/// Stores the result for a test suite.
#[derive(Debug, Clone, Copy, PartialEq, Deserialize)]
pub struct SuiteResultEvent
{
    /// The number of tests passed in the suite.
    pub passed: Option<u64>,

    /// The number of tests failed in the suite.
    pub failed: Option<u64>,

    /// The number of tests ignored in the suite.
    pub ignored: Option<u64>,

    /// The number of tests measured in the suite.
    pub measured: Option<u64>,

    /// The number of tests filtered out in the suite.
    pub filtered_out: Option<u64>,

    /// The time taken to execute the suite.
    pub exec_time: Option<f64>
}

/// Tests for the [SuiteResultEvent].
#[cfg(test)]
mod tests
{
    use serde_json::json;
    use super::super::enums::suite_event::SuiteEvent;
    use super::super::super::enums::event::Event;

    /// Tests the parsing of a suite result event.
    #[test]
    fn test_parse_suite_result_event()
    {
        let json = json!(
        {
            "type": "suite",
            "event": "ok",
            "passed": 1,
            "failed": 0,
            "ignored": 1,
            "measured": 1,
            "filtered_out": 1,
            "exec_time": 1.0
        });

        let event: Event = serde_json::from_value(json).unwrap();
        assert!(matches!(event, Event::Suite(SuiteEvent::Ok(_))))
    }
}