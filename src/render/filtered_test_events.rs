/*!
 * Stores the test events filtered based on tab.
 */
use super::super::commands::structures::enums::event::Event;
use super::super::commands::structures::suite::enums::suite_event::SuiteEvent;
use super::super::commands::structures::test::enums::test_event::TestEvent;

/// Stores the test events filtered based on tab.
pub struct FilteredTestEvents<'a>
{
    /// The collection of all [TestEvent] excluding [TestEvent::Started].
    pub all: Vec<&'a Event>,

    /// The collection of [TestEvent::Ok].
    pub passed: Vec<&'a Event>,

    /// The collection of [TestEvent::Failed].
    pub failed: Vec<&'a Event>,

    /// The collection of [TestEvent::Ignored].
    pub ignored: Vec<&'a Event>,

    /// The time taken to execute the test suite.
    pub execute_time: f64
}

/// The implementation of the [FilteredTestEvents].
impl<'a> FilteredTestEvents<'a>
{
    /// Retrieves the events based on the selected tab.
    /// # Arguments
    /// * `current_tab_index` - The index of the current tab.
    /// # Returns
    /// `&[&Event]` - The collection of events filtered by the selected tab.
    pub fn get_events_by_tab_index(&self, current_tab_index: usize) -> &[&Event]
    {
        match current_tab_index
        {
            1 => &self.passed,
            2 => &self.failed,
            3 => &self.ignored,
            _ => &self.all
        }
    }
}

/// Filters the events based on the selected tab.
/// # Arguments
/// * `events` - The collection of events to filter.
/// # Returns
/// `FilteredEvents` - The filtered events by selected tab.
pub fn filter_events(events: &[Event]) -> FilteredTestEvents<'_>
{
    let mut all = Vec::new();
    let mut passed = Vec::new();
    let mut failed = Vec::new();
    let mut ignored = Vec::new();
    let mut execute_time = 0.0;

    for event in events
    {
        if matches!(event, Event::Test(_)) && !matches!(event, Event::Test(TestEvent::Started(_)))
        {
            all.push(event);
        }

        match event
        {
            Event::Test(TestEvent::Ok(_)) => passed.push(event),
            Event::Test(TestEvent::Failed(_)) => failed.push(event),
            Event::Test(TestEvent::Ignored(_)) => ignored.push(event),
            Event::Suite(SuiteEvent::Ok(ok_event)) => execute_time = ok_event.exec_time.unwrap_or_default(),
            Event::Suite(SuiteEvent::Failed(failed_event)) => execute_time = failed_event.exec_time.unwrap_or_default(),
            _ => {}
        }
    }

    FilteredTestEvents
    {
        all,
        passed,
        failed,
        ignored,
        execute_time
    }
}

/// Tests for the [FilteredBenchEvents].
#[cfg(test)]
mod tests
{
    use super::super::super::commands::structures::bench::enums::bench_event::BenchEvent;
    use super::super::super::commands::structures::test::enums::test_event::TestEvent;
    use super::super::super::commands::structures::test::result::TestResultEvent;
    use super::super::super::commands::structures::test::started::TestStartedEvent;
    use super::*;

    /// Tests the [crate::render::filtered_bench_events::filter_events] function to assert filtering is correct.
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
            })),
            Event::Test(TestEvent::Ok(TestResultEvent
            {
                name: "test_test_ok".to_string(),
                exec_time: Some(1.0),
                stdout: Some("test_stdout".to_string())
            })),
            Event::Test(TestEvent::Ok(TestResultEvent
            {
                name: "test_test_ok".to_string(),
                exec_time: Some(1.0),
                stdout: Some("test_stdout".to_string())
            })),
            Event::Test(TestEvent::Failed(TestResultEvent
            {
                name: "test_test_failed".to_string(),
                exec_time: Some(1.0),
                stdout: Some("test_stdout".to_string())
            }))
        ];

        let filtered_events = filter_events(&events);

        assert_eq!(filtered_events.all.len(), 3);
        assert_eq!(filtered_events.passed.len(), 2);
        assert_eq!(filtered_events.failed.len(), 1);
    }
}