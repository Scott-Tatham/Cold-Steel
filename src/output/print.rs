/*!
 * Handles the printing of events.
 */
use crossterm::style::{Color, Stylize};
use ratatui::backend::IntoCrossterm;
use crate::commands::structures::bench::enums::bench_event::BenchEvent;
use crate::commands::structures::test::enums::test_event::TestEvent;
use crate::text::renderer;
use super::super::commands::structures::enums::event::Event;
use super::super::commands::structures::enums::r#type::CommandType;
use super::super::commands::structures::suite::enums::suite_event::SuiteEvent;
use super::super::configuration::core_configuration::Configuration;
use super::super::configuration::theme::Theme;

/// Prints the events when the '--print' flag is used.
/// # Arguments
/// * `events` - The collection of events to print.
/// * `configuration` - The configuration of Cold Steel.
/// * `command_type` - The type of command used.
pub fn print_events(events: &[Event], configuration: &Configuration, command_type: CommandType)
{
    let theme_name = configuration.theme.clone().unwrap_or("default".to_string());
    let theme = configuration.themes.get(theme_name.as_str());

    for event in events
    {
        match event
        {
            Event::Suite(suite_event) => print_suite_event(suite_event, theme, command_type),
            Event::Test(test_event) => print_test_event(test_event, theme, command_type),
            Event::Bench(bench_event) => print_bench_event(bench_event, theme),
        }
    }
}

/// Prints a [SuiteEvent].
/// # Arguments
/// * `suite_event` - The [SuiteEvent] to print.
/// * `theme` - The theme to use for styling the output.
/// * `command_type` - The type of command used.
fn print_suite_event(suite_event: &SuiteEvent, theme: Option<&Theme>, command_type: CommandType)
{
    let background_colour = theme.map(|theme| theme.background).unwrap_or(ratatui::style::Color::Reset).into_crossterm();
    let text_colour = theme.map(|theme| theme.text).unwrap_or(ratatui::style::Color::Reset).into_crossterm();
    let passed_colour = theme.map(|theme| theme.passed).unwrap_or(ratatui::style::Color::Reset).into_crossterm();
    let failed_colour = theme.map(|theme| theme.failed).unwrap_or(ratatui::style::Color::Reset).into_crossterm();

    match suite_event
    {
        SuiteEvent::Started(suite_started_event) =>
        {
            match command_type
            {
                CommandType::Test { .. } =>
                {
                    let event_count_text = suite_started_event.test_count.unwrap_or_default().to_string();

                    println!("{}{}", "Test Suite Started: ".with(text_colour).on(background_colour), event_count_text.with(text_colour).on(background_colour));
                },
                CommandType::Bench { .. } =>
                {
                    println!("{}", "Bench Suite Started:".with(text_colour).on(background_colour));
                }
            }
        },
        SuiteEvent::Ok(suite_ok_event) =>
        {
            let measured_event_count_text = suite_ok_event.measured.unwrap_or_default().to_string();
            let passed_event_count_text = suite_ok_event.passed.unwrap_or_default().to_string();
            let ignored_event_count_text = suite_ok_event.ignored.unwrap_or_default().to_string();
            let execution_time_text = suite_ok_event.exec_time.unwrap_or_default().to_string();

            match command_type
            {
                CommandType::Test { .. } =>
                {
                    println!("{}{}{}{}{}{}{}{}{}", "Suite ".with(text_colour).on(background_colour), "Passed ".with(passed_colour).on(background_colour), "-".with(text_colour).on(background_colour), " Passed: ".with(text_colour).on(background_colour), passed_event_count_text.with(passed_colour).on(background_colour), " Ignored: ".with(text_colour).on(background_colour), ignored_event_count_text.with(text_colour).on(background_colour), " Execution Time: ".with(text_colour).on(background_colour), execution_time_text.with(text_colour).on(background_colour));
                },
                CommandType::Bench { .. } =>
                {
                    print_suite_complete_bench(&measured_event_count_text, &execution_time_text, text_colour, background_colour);
                }
            }
        },
        SuiteEvent::Failed(suite_failed_event) =>
        {
            let measured_event_count_text = suite_failed_event.measured.unwrap_or_default().to_string();
            let passed_event_count_text = suite_failed_event.passed.unwrap_or_default().to_string();
            let failed_event_count_text = suite_failed_event.failed.unwrap_or_default().to_string();
            let ignored_event_count_text = suite_failed_event.ignored.unwrap_or_default().to_string();
            let execution_time_text = suite_failed_event.exec_time.unwrap_or_default().to_string();

            match command_type
            {
                CommandType::Test { .. } =>
                {
                    println!("{}{}{}{}{}{}{}{}{}{}{}", "Suite ".with(text_colour).on(background_colour), "Failed ".with(failed_colour).on(background_colour), "-".with(text_colour).on(background_colour), " Passed: ".with(text_colour).on(background_colour), passed_event_count_text.with(passed_colour).on(background_colour), " Failed: ".with(text_colour).on(background_colour), failed_event_count_text.with(failed_colour).on(background_colour), " Ignored: ".with(text_colour).on(background_colour), ignored_event_count_text.with(text_colour).on(background_colour), " Execution Time: ".with(text_colour).on(background_colour), execution_time_text.with(text_colour).on(background_colour));
                },
                CommandType::Bench { .. } =>
                {
                    print_suite_complete_bench(&measured_event_count_text, &execution_time_text, text_colour, background_colour);
                }
            }
        }
    }
}

/// Prints a [TestEvent].
/// # Arguments
/// * `test_event` - The [TestEvent] to print.
/// * `theme` - The theme to use for styling the output.
/// * `command_type` - The type of command used.
fn print_test_event(test_event: &TestEvent, theme: Option<&Theme>, command_type: CommandType)
{
    let background_colour = theme.map(|theme| theme.background).unwrap_or(ratatui::style::Color::Reset).into_crossterm();
    let text_colour = theme.map(|theme| theme.text).unwrap_or(ratatui::style::Color::Reset).into_crossterm();
    let passed_colour = theme.map(|theme| theme.passed).unwrap_or(ratatui::style::Color::Reset).into_crossterm();
    let failed_colour = theme.map(|theme| theme.failed).unwrap_or(ratatui::style::Color::Reset).into_crossterm();

    let timings = match command_type
    {
        CommandType::Test { timings, .. } => timings,
        _ => false
    };

    match test_event
    {
        TestEvent::Started(_) => {},
        TestEvent::Ok(test_ok_event) =>
        {
            let test_ok_name_text = test_ok_event.name.clone();
            let test_ok_execution_time_text = test_ok_event.exec_time.unwrap_or_default().to_string();

            if timings
            {
                println!("{}{}{}{}{}", renderer::render_text("\t  ", "\t✔ ", "\tV ").with(passed_colour).on(background_colour), " Execution Time: ".with(text_colour).on(background_colour), test_ok_execution_time_text.with(text_colour).on(background_colour), " ".with(text_colour).on(background_colour), test_ok_name_text.with(text_colour).on(background_colour));

                return;
            }

            println!("{}{}", renderer::render_text("\t  ", "\t✔ ", "\tV ").with(passed_colour).on(background_colour), test_ok_name_text.with(text_colour).on(background_colour));
        },
        TestEvent::Failed(test_failed_event) =>
        {
            let test_failed_name_text = test_failed_event.name.clone();
            let test_failed_execution_time_text = test_failed_event.exec_time.unwrap_or_default().to_string();
            let test_failed_stdout_text = test_failed_event.stdout.clone().unwrap_or_default().to_string();
            
            if timings
            {
                println!("{}{}{}{}", renderer::render_text("\t  ", "\t✗ ", "\tX ").with(failed_colour).on(background_colour), " Execution Time: ".with(text_colour).on(background_colour), test_failed_execution_time_text.with(text_colour).on(background_colour), test_failed_name_text.with(text_colour).on(background_colour));
            }
            
            else
            {
                println!("{}{}", renderer::render_text("\t  ", "\t✗ ", "\tX ").with(failed_colour).on(background_colour), test_failed_name_text.with(text_colour).on(background_colour));
            }

            println!("{}{}", "\t\t".with(text_colour).on(background_colour) ,test_failed_stdout_text.with(text_colour).on(background_colour));
        },
        TestEvent::Ignored(test_ignored_event) =>
        {
            if matches!(command_type, CommandType::Test { .. })
            {
                let test_ignored_name_text = test_ignored_event.name.clone();

                println!("{}{}", renderer::render_text("\t  ", "\t‐ ", "\t- ").with(text_colour).on(background_colour), test_ignored_name_text.with(text_colour).on(background_colour));
            }
        },
    }
}

/// Prints a [BenchEvent].
/// # Arguments
/// * `test_event` - The [BenchEvent] to print.
/// * `theme` - The theme to use for styling the output.
fn print_bench_event(bench_event: &BenchEvent, theme: Option<&Theme>)
{
    let background_colour = theme.map(|theme| theme.background).unwrap_or(ratatui::style::Color::Reset).into_crossterm();
    let text_colour = theme.map(|theme| theme.text).unwrap_or(ratatui::style::Color::Reset).into_crossterm();

    let bench_event_name_text = bench_event.name.clone();
    let bench_event_median_text = bench_event.median.unwrap_or_default().to_string();
    let bench_event_deviation_text = bench_event.deviation.unwrap_or_default().to_string();

    println!("{}{}{}{}{}{}", "\tMedian: ".with(text_colour).on(background_colour), bench_event_median_text.with(text_colour).on(background_colour), " Deviation: ".with(text_colour).on(background_colour), bench_event_deviation_text.with(text_colour).on(background_colour), " ".with(text_colour).on(background_colour), bench_event_name_text.with(text_colour).on(background_colour))
}

/// Print the bench suite complete event.
/// # Arguments
/// * `passed_event_count_text` - The count of passed events.
/// * `execution_time_text` - The execution time of the suite.
/// * `text_colour` - The text colour to use for styling the output.
/// * `background_colour` - The background colour to use for styling the output.
fn print_suite_complete_bench(passed_event_count_text: &str, execution_time_text: &str, text_colour: Color, background_colour: Color)
{
    println!("{}{}{}{}{}{}", "Suite Complete ".with(text_colour).on(background_colour), "-".with(text_colour).on(background_colour), " All: ".with(text_colour).on(background_colour), passed_event_count_text.with(text_colour).on(background_colour), " Execution Time: ".with(text_colour).on(background_colour), execution_time_text.with(text_colour).on(background_colour));
}