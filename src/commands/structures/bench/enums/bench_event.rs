/*!
 * Defines the types of bench events that are rendered.
 */
use serde::Deserialize;

/// Defines the types of bench events that are rendered.
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct BenchEvent
{
    /// The name of the benchmark.
    pub name: String,

    /// The median time in nanoseconds taken to execute the benchmark.
    pub median: Option<f64>,

    /// The deviation time in nanoseconds of the benchmark.
    pub deviation: Option<f64>
}

/// Tests for the [BenchEvent].
#[cfg(test)]
mod tests
{
    use serde_json::json;
    use super::super::super::super::enums::event::Event;

    /// Tests the parsing of a bench event.
    #[test]
    fn test_parse_bench_event()
    {
        let json = json!(
        {
            "type": "bench",
            "name": "test_bench",
            "median": 1.0,
            "deviation": 1.0
        });

        let event: Event = serde_json::from_value(json).unwrap();
        assert!(matches!(event, Event::Bench(_)))
    }
}