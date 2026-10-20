/*!
 * The entry point for Cold Steel.
 */
extern crate core;

pub mod commands;
pub mod render;
pub mod text;
pub mod configuration;
pub mod output;

use std::io::Error;
use std::time::{Duration, Instant};
use clap::Parser;
use crossterm::event;
use crossterm::event::{KeyCode, KeyEventKind};
use commands::arguments::Arguments;
use commands::events::CommandEvents;
use commands::structures::enums::event::Event;
use commands::structures::enums::r#type::CommandType;
use configuration::core_configuration;
use render::bench::BenchRenderer;
use render::renderer::Renderer;
use render::test::TestRender;
use output::print;

/**
 * The minimum interval between renders.
 */
const MINIMUM_RENDER_INTERVAL: u64 = 100;

/**
 * The minimum interval between polling.
 */
const MINIMUM_POLL_INTERVAL: u64 = 50;

/**
 * The entry point for Cold Steel.
 */
fn main() -> Result<(), Error>
{
    let arguments = Arguments::parse();
    let print_output: bool;
    let configuration = core_configuration::load_or_create_configuration();

    let (raw_events, mut renderer) = match arguments.command
    {
        CommandType::Test { timings, print } =>
        {
            let mut arguments = vec!["test", "--", "--Z", "unstable-options", "--format", "json"];

            if timings
            {
                arguments.push("--report-time");
            }

            let result = commands::command::build_command(arguments);
            let raw_events = CommandEvents::new(&mut result?)?;
            let test_renderer = TestRender::new(timings).expect("Failed to create an instance of TestRender.");
            let renderer = Box::new(test_renderer) as Box<dyn Renderer>;
            print_output = print;

            (raw_events, renderer)
        },
        CommandType::Bench { print } =>
        {
            let arguments = vec!["bench", "--", "--Z", "unstable-options", "--format", "json"];
            let result = commands::command::build_command(arguments);
            let raw_events = CommandEvents::new(&mut result?)?;
            let bench_render = BenchRenderer::new().expect("Failed to create an instance of BenchRenderer.");
            let renderer = Box::new(bench_render) as Box<dyn Renderer>;
            print_output = print;

            (raw_events, renderer)
        }
    };

    let minimum_render_interval = Duration::from_millis(MINIMUM_RENDER_INTERVAL);
    let mut last_render = Instant::now();
    let mut events: Vec<Event> = Vec::default();

    for raw_event in raw_events
    {
        events.push(raw_event);

        if last_render.elapsed() >= minimum_render_interval
        {
            renderer.render(&configuration, events.as_slice(), KeyCode::Null)?;
            last_render = Instant::now();
        }
    }
    
    renderer.render(&configuration, events.as_slice(), KeyCode::Null)?;

    loop
    {
        if event::poll(Duration::from_millis(MINIMUM_POLL_INTERVAL))?
        {
            if let event::Event::Key(key) = event::read()?
            {
                if key.kind == KeyEventKind::Press
                {
                    match key.code
                    {
                        KeyCode::Esc => break,
                        _ => renderer.render(&configuration, events.as_slice(), key.code)?
                    }
                }
            }
        }
    }
    
    renderer.exit()?;

    if print_output
    {
        print::print_events(&events, &configuration, arguments.command);
    }

    Ok(())
}