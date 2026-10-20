/*!
 * Stores the bench events filtered.
 */
use super::super::commands::structures::enums::event::Event;
use super::super::commands::structures::suite::enums::suite_event::SuiteEvent;

/// Stores the bench events filtered.
pub struct FilteredBenchEvents<'a>
{
    /// The collection of all [BenchEvent].
    pub all: Vec<&'a Event>,

    /// The time taken to execute the benchmark.
    pub execute_time: f64
}

/// Filters the events.
/// # Arguments
/// * `events` - The collection of events to filter.
/// # Returns
/// `FilteredEvents` - The filtered events.
pub fn filter_events(events: &[Event]) -> FilteredBenchEvents<'_>
{
    let mut all: Vec<&Event> = Vec::default();
    let mut execute_time = 0.0;

    for event in events
    {
        if matches!(event, Event::Bench(_))
        {
            all.push(event);
        }

        match event
        {
            Event::Suite(SuiteEvent::Ok(ok_event)) => execute_time = ok_event.exec_time.unwrap_or_default(),
            Event::Suite(SuiteEvent::Failed(failed_event)) => execute_time = failed_event.exec_time.unwrap_or_default(),
            _ => {}
        }
    }

    FilteredBenchEvents
    {
        all,
        execute_time
    }
}

/// Tests for the [FilteredBenchEvents].
#[cfg(test)]
mod tests
{
    use super::super::super::commands::structures::bench::enums::bench_event::BenchEvent;
    use super::super::super::commands::structures::test::enums::test_event::TestEvent;
    use super::super::super::commands::structures::test::started::TestStartedEvent;
    use super::*;

    /// Tests the [filter_events] function to assert filtering is correct.
    #[test]
    fn test_filter_events()
    {
        let events = vec![
            Event::Bench(BenchEvent
            {
                name: "test_bench".to_string(),
                median: Some(1.0),
                deviation: Some(1.0)
            }),
            Event::Test(TestEvent::Started(TestStartedEvent
            {
                name: "test_test_started".to_string()
            }))
        ];

        let filtered_events = filter_events(&events);

        assert_eq!(filtered_events.all.len(), 1);
    }
}