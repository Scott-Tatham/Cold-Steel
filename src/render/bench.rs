/*!
 * Handles the rendering of 'cargo bench'.
 */
use std::io::Error;
use crossterm::event::KeyCode;
use ratatui::layout::{Constraint, Direction};
use ratatui::text::Span;
use ratatui::widgets::{Borders, ListItem, ListState, Padding, Wrap};
use crate::configuration::core_configuration::Configuration;
use super::super::commands::structures::enums::event;
use super::super::commands::structures::enums::event::Event;
use super::super::render::filtered_bench_events;
use super::super::text::renderer;
use super::renderer::{Renderer, RendererData};

/// The constraints for the main layout.
const MAIN_LAYOUT_CONSTRAINTS: [Constraint; 3] = [Constraint::Length(3), Constraint::Fill(1), Constraint::Length(3)];

/// The padding for the output block.
const OUTPUT_BLOCK_PADDING: Padding = Padding::horizontal(1);

/// Renders the 'cargo bench' output.
#[derive(Debug)]
pub struct BenchRenderer
{
    /// The common data for rendering.
    pub render_data: RendererData,

    /// The index for the current bench selected.
    pub current_bench_index: usize
}

/// The implementation of the [BenchRenderer].
impl BenchRenderer
{
    /// Initialises a new instance of the [BenchRenderer].
    /// # Returns
    /// `Result<BenchRenderer, Error>` - The bench renderer or an error.
    /// # Errors
    /// Returns an error if the renderer data cannot be created.
    pub fn new() -> Result<Self, Error>
    {
        Ok(Self
        {
            render_data: RendererData::new()?,
            current_bench_index: 0
        })
    }

    /// Processes the input action.
    /// # Arguments
    /// * `input_action` - The input action to process.
    fn process_input(&mut self, input_action: KeyCode)
    {
        match input_action
        {
            KeyCode::Char('w') | KeyCode::Up => self.current_bench_index = self.current_bench_index - 1,
            KeyCode::Char('s') | KeyCode::Down => self.current_bench_index = self.current_bench_index + 1,
            _ => {}
        }
    }
}

/// The implementation of the [Renderer] trait for the [BenchRenderer].
impl Renderer for BenchRenderer
{
    /// Retrieve the instance of the renderer data.
    /// # Returns
    /// `&RendererData` - The renderer data.
    fn render_data(&self) -> &RendererData
    {
        &self.render_data
    }

    /// Retrieve the mutable instance of the renderer data.
    /// # Returns
    /// `&mut RendererData` - The mutable renderer data.
    fn render_data_mutable(&mut self) -> &mut RendererData
    {
        &mut self.render_data
    }

    /// Renders the UI every update of the command output stream.
    /// # Arguments
    /// * `configuration` - The configuration of Cold Steel.
    /// * `events` - The collection of events being rendered.
    /// * `input_action` - The input action that invoked the render.
    /// # Returns
    /// `Result<(), Error>` - The result of the render or an error.
    /// # Errors
    /// Returns an error if the terminal cannot be drawn.
    fn render(&mut self, configuration: &Configuration, events: &[Event], input_action: KeyCode) -> Result<(), Error>
    {
        self.process_input(input_action);
        
        let filtered_events = filtered_bench_events::filter_events(events);

        let event_list = filtered_events.all.as_slice();
        let event_list_count = event_list.len();

        self.current_bench_index = super::renderer::wrap_index(self.current_bench_index, event_list_count);
        let current_bench_index = self.current_bench_index;

        let theme_name = configuration.theme.clone().unwrap_or("default".to_string());
        let theme = configuration.themes.get(theme_name.as_str());

        let background_style = super::renderer::build_background_style(theme);
        let text_style = super::renderer::build_text_style(theme);
        let title_style = super::renderer::build_title_style(theme);
        let highlight_style = super::renderer::build_highlight_style(theme);
        let border_style = super::renderer::build_border_style(theme);

        self.render_data_mutable().terminal.draw(|frame|
        {
            let main_layout = super::renderer::build_layout(Direction::Vertical, MAIN_LAYOUT_CONSTRAINTS, frame.area());

            let summary_block = super::renderer::build_summary_block(renderer::render_text("󱘗 Rust Bench", "Rust Bench", "Rust Bench"), background_style, title_style, border_style);
            let summary_text = format!("All: {} | Execution Time: {}", event_list_count, filtered_events.execute_time);
            let summary_text_paragraph = super::renderer::build_paragraph(summary_text, Wrap { trim: true }, text_style, summary_block);
            frame.render_widget(summary_text_paragraph, main_layout[0]);

            let event_list_count_text = format!("All: {}", event_list_count);
            let control_text = renderer::render_text("(List)  = [W/S]", "(List) ↑↓ = [W/S]", "(List) ^v = [W/S]");
            let bench_block = super::renderer::build_list_block("Benchmarks".to_string(), event_list_count_text, control_text, event_list, current_bench_index, background_style, title_style, border_style);

            let mut list_state = ListState::default();
            list_state.select(Some(current_bench_index));

            let list = super::renderer::build_list(event_list.iter().map(|raw_event|
            {
                let event_name = format!(" {}", event::determine_event_name(raw_event));
                let event_name_span = Span::styled(event_name, text_style);
                ListItem::new(event_name_span)
            }).collect::<Vec<ListItem>>(), bench_block, highlight_style);

            frame.render_stateful_widget(list, main_layout[1], &mut list_state);

            let output_block = super::renderer::build_block_with_title("Output".to_string(), OUTPUT_BLOCK_PADDING, Borders::ALL, background_style, title_style, border_style);

            let mut bench_data = String::default();
            
            if event_list.len() > 0
            {
                bench_data = match Some(event_list[current_bench_index])
                {
                    Some(Event::Bench(bench_event)) => format!("Median: {} Deviation: {}", bench_event.median.unwrap_or_default(), bench_event.deviation.unwrap_or_default()),
                    _ => bench_data
                };
            }

            let output = super::renderer::build_paragraph(bench_data, Wrap { trim: true }, text_style, output_block);
            frame.render_widget(output, main_layout[2]);
        })?;

        Ok(())
    }
}