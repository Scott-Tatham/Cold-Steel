/*!
 * Defines the types of events that are rendered.
 */
use serde::Deserialize;
use super::super::bench::enums::bench_event::BenchEvent;
use super::super::suite::enums::suite_event::SuiteEvent;
use super::super::test::enums::test_event::TestEvent;

/// Defines the types of events that are rendered.
#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum Event
{
    /// The resulting event for a test suite.
    Suite(SuiteEvent),

    /// The resulting event for a test case.
    Test(TestEvent),

    /// The resulting event for a benchmark.
    Bench(BenchEvent)
}

/// Determines the name of the specified event.
/// # Arguments
/// * `event` - The event being rendered.
/// # Returns
/// `String` - The name of the event.
pub fn determine_event_name(event: &Event) -> String
{
    match event
    {
        Event::Test(TestEvent::Started(event)) => event.name.clone(),
        Event::Test(TestEvent::Ok(event)) => event.name.clone(),
        Event::Test(TestEvent::Failed(event)) => event.name.clone(),
        Event::Test(TestEvent::Ignored(event)) => event.name.clone(),
        Event::Bench(event) => event.name.clone(),
        _ => String::default()
    }
}

/// Tests for the [determine_event_name] function.
#[cfg(test)]
mod tests
{
    use super::*;
    use super::super::super::test::enums::test_event::TestEvent;
    use super::super::super::test::started::TestStartedEvent;

    /// Tests the [determine_event_name] function.
    #[test]
    fn test_determine_event_name_test()
    {
        let event_name = determine_event_name(&Event::Test(TestEvent::Started(TestStartedEvent
        {
            name: "test_started".to_string()
        })));
        
        assert_eq!(event_name, "test_started");
    }
}