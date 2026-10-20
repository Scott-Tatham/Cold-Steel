/*!
 * Handles the rendering of 'cargo test'.
 */
use core::f64;
use std::io::Error;
use crossterm::event::KeyCode;
use ratatui::layout::{Constraint, Direction};
use ratatui::style::Style;
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, ListItem, ListState, Padding, Tabs, Wrap};
use super::super::commands::structures::enums::event;
use super::super::commands::structures::enums::event::Event;
use super::super::commands::structures::test::enums::test_event::TestEvent;
use super::super::configuration::core_configuration::Configuration;
use super::super::render::filtered_test_events;
use super::super::text::renderer;
use super::renderer::{Renderer, RendererData};

/// The tabs to display.
const TABS: [&str; 4] = ["All", "Passed", "Failed", "Ignored"];

/// The constraints for the tests layout.
const TESTS_LAYOUT_CONSTRAINTS: [Constraint; 2] = [Constraint::Length(3), Constraint::Fill(1)];

/// The padding for the test tabs block.
const TEST_TABS_BLOCK_PADDING: Padding = Padding::new(1, 1, 1, 0);

/// The padding for the list block.
const TEST_LIST_BLOCK_PADDING: Padding = Padding::new(1, 2, 0, 1);

/// The padding for the expected output block.
const EXPECTED_OUTPUT_BLOCK_PADDING: Padding = Padding::horizontal(1);

/// The padding for the actual output block.
const ACTUAL_OUTPUT_BLOCK_PADDING: Padding = Padding::horizontal(1);

/// The padding for the output block.
const OUTPUT_BLOCK_PADDING: Padding = Padding::horizontal(1);

/// Renders the 'cargo test' output.
#[derive(Debug)]
pub struct TestRender
{
    /// The common data for rendering.
    pub render_data: RendererData,

    /// The index for the current tab selected.
    pub current_tab_index: usize,

    /// The index for the current test selected.
    pub current_test_index: usize,

    /// The flag to determine whether to display timings for tests.
    pub timings: bool
}

/// The implementation of the [TestRender].
impl TestRender
{
    /// Initialises a new instance of the [TestRender].
    /// # Returns
    /// `Result<TestRender, Error>` - The test render or an error.
    /// # Errors
    /// Returns an error if the TestRender cannot be created.
    pub fn new(timings: bool) -> Result<Self, Error>
    {
        Ok(Self
        {
            render_data: RendererData::new()?,
            current_tab_index: 0,
            current_test_index: 0,
            timings
        })
    }

    /// Processes the input action.
    /// # Arguments
    /// * `input_action` - The input action to process.
    fn process_input(&mut self, input_action: KeyCode)
    {
        match input_action
        {
            KeyCode::Char('a') | KeyCode::Left =>
                {
                    self.current_tab_index = self.current_tab_index - 1;
                    self.current_test_index = 0;
                },
            KeyCode::Char('d') | KeyCode::Right =>
                {
                    self.current_tab_index = self.current_tab_index + 1;
                    self.current_test_index = 0;
                },
            KeyCode::Char('w') | KeyCode::Up => self.current_test_index = self.current_test_index - 1,
            KeyCode::Char('s') | KeyCode::Down => self.current_test_index = self.current_test_index + 1,
            _ => {}
        }
    }

    /// Builds the tabs for the test list.
    /// # Arguments
    /// * `current_tab_index` - The index of the current tab.
    /// * `text_style` - The style to use for text.
    /// * `highlight_style` - The style to use for the highlight.
    /// * `test_tabs_block` - The block to use for the tabs.
    /// # Returns
    /// `Tabs` - Returns the tabs.
    fn build_tabs(current_tab_index: usize, text_style: Style, highlight_style: Style, test_tabs_block: Block<'_>) -> Tabs<'_>
    {
        let tabs = vec![Span::styled(TABS[0], text_style), Span::styled(TABS[1], text_style), Span::styled(TABS[2], text_style), Span::styled(TABS[3], text_style)];
        let divider_symbol = "|";
        let divider_symbol_span = Span::styled(divider_symbol, text_style);

        Tabs::new(tabs)
            .select(current_tab_index)
            .highlight_style(highlight_style)
            .block(test_tabs_block)
            .divider(divider_symbol_span)
    }

    /// Builds an item for the list.
    /// # Arguments
    /// * `raw_event` - The event to create the list item for.
    /// * `current_tab_index` - The index of the current tab.
    /// * `timings` - The flag to determine whether to display timings for tests.
    /// * `text_style` - The style to use for text.
    /// * `passed_style` - The style to use for passed events.
    /// * `failed_style` - The style to use for failed events.
    /// # Returns
    /// `ListItem` - The list item.
    fn build_list_item(raw_event: &Event, current_tab_index: usize, timings: bool, text_style: Style, passed_style: Style, failed_style: Style) -> ListItem<'_>
    {
        let event_name = format!(" {}", event::determine_event_name(raw_event));
        let event_name_span = Span::styled(event_name, text_style);

        let (prefix_symbol, time) = match raw_event
        {
            Event::Test(TestEvent::Ok(event)) => (Span::styled(format!(" {}", renderer::render_text(" ", "✔", "V")), passed_style), Span::styled(format!(" {}", event.exec_time.unwrap_or_default()), text_style)),
            Event::Test(TestEvent::Failed(event)) => (Span::styled(format!(" {}", renderer::render_text(" ", "✗", "X")), failed_style), Span::styled(format!(" {}", event.exec_time.unwrap_or_default()), text_style)),
            _ => (Span::styled(format!(" {}", renderer::render_text(" ", "‐", "-")), text_style), Span::styled(format!(" {}", f64::default()), text_style))
        };

        match current_tab_index
        {
            0 =>
            {
                let mut list_item_text = Line::from(prefix_symbol);

                if timings
                {
                    list_item_text.push_span(time);
                }

                list_item_text.push_span(event_name_span);

                ListItem::new(list_item_text)
            },
            1 | 2 =>
            {
                let mut list_item_text = Line::default();

                if timings
                {
                    list_item_text.push_span(time);
                }

                list_item_text.push_span(event_name_span);

                ListItem::new(list_item_text)
            },
            _ =>
            {
                let list_item_text = Line::from(event_name_span);

                ListItem::new(list_item_text)
            }
        }
    }
}

/// The implementation of the [Renderer] for [TestRender].
impl Renderer for TestRender
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
    /// Returns an error if the frame cannot be drawn.
    fn render(&mut self, configuration: &Configuration, events: &[Event], input_action: KeyCode) -> Result<(), Error>
    {
        self.process_input(input_action);

        let timings = self.timings;

        let filtered_events = filtered_test_events::filter_events(events);

        self.current_tab_index = super::renderer::wrap_index(self.current_tab_index, TABS.len());

        let all_count = filtered_events.all.len();
        let passed_count = filtered_events.passed.len();
        let failed_count = filtered_events.failed.len();
        let ignored_count = filtered_events.ignored.len();

        self.current_test_index = match self.current_tab_index
        {
            1 => super::renderer::wrap_index(self.current_test_index, passed_count),
            2 => super::renderer::wrap_index(self.current_test_index, failed_count),
            3 => super::renderer::wrap_index(self.current_test_index, ignored_count),
            _ => super::renderer::wrap_index(self.current_test_index, all_count)
        };

        let current_tab_index = self.current_tab_index;
        let current_test_index = self.current_test_index;

        let event_list = filtered_events.get_events_by_tab_index(current_tab_index);
        let event_list_count = event_list.len();

        let is_failed_event = matches!(event_list.get(current_test_index), Some(Event::Test(TestEvent::Failed(_))));

        let theme_name = configuration.theme.clone().unwrap_or("default".to_string());
        let theme = configuration.themes.get(theme_name.as_str());

        let background_style = super::renderer::build_background_style(theme);
        let text_style = super::renderer::build_text_style(theme);
        let title_style = super::renderer::build_title_style(theme);
        let passed_style = super::renderer::build_passed_style(theme);
        let failed_style = super::renderer::build_failed_style(theme);
        let highlight_style = super::renderer::build_highlight_style(theme);
        let border_style = super::renderer::build_border_style(theme);

        self.render_data_mutable().terminal.draw(|frame|
        {
            let mut main_layout_constraints: Vec<Constraint> = vec![Constraint::Length(3), Constraint::Fill(1)];

            if is_failed_event
            {
                main_layout_constraints.push(Constraint::Length(9));
            }

            let main_layout = super::renderer::build_layout(Direction::Vertical, main_layout_constraints, frame.area());

            let tests_layout = super::renderer::build_layout(Direction::Vertical, TESTS_LAYOUT_CONSTRAINTS, main_layout[1]);

            let summary_block = super::renderer::build_summary_block(renderer::render_text("󱘗 Rust Test", "Rust Test", "Rust Test"), background_style, title_style, border_style);
            let summary_text = format!("All: {} | Passed: {} | Failed: {} | Ignored: {} | Execution Time: {}", all_count, passed_count, failed_count, ignored_count, filtered_events.execute_time);
            let summary_paragraph = super::renderer::build_paragraph(summary_text, Wrap { trim: true }, text_style, summary_block);
            frame.render_widget(summary_paragraph, main_layout[0]);

            let event_list_count_text = format!("{}: {}", TABS[current_tab_index], event_list_count);
            let control_text = renderer::render_text("(List)  = [W/S] | (Tabs)   = [A/D]", "(List) ↑↓ = [W/S] | (Tabs) ←→ = [A/D]", "(List) ^v = [W/S] | (Tabs) <> = [A/D]");
            let test_block = super::renderer::build_list_block("Tests".to_string(), event_list_count_text, control_text, event_list, current_test_index, background_style, title_style, border_style);
            frame.render_widget(test_block, main_layout[1]);

            let test_tabs_block = super::renderer::build_block(TEST_TABS_BLOCK_PADDING, Borders::NONE, background_style, title_style, border_style);
            let test_tabs = Self::build_tabs(current_tab_index, text_style, highlight_style, test_tabs_block);
            frame.render_widget(test_tabs, tests_layout[0]);

            let test_list_block = super::renderer::build_block(TEST_LIST_BLOCK_PADDING, Borders::NONE, background_style, title_style, border_style);
            let mut list_state = ListState::default();
            list_state.select(Some(current_test_index));

            let list = super::renderer::build_list(event_list.iter().map(|raw_event|
            {
                Self::build_list_item(raw_event, current_tab_index, timings, text_style, passed_style, failed_style)
            }).collect::<Vec<ListItem>>(), test_list_block, highlight_style);

            frame.render_stateful_widget(list, tests_layout[1], &mut list_state);

            if is_failed_event && event_list_count > 0
            {
                if let Event::Test(TestEvent::Failed(event)) = event_list[current_test_index].clone()
                {
                    let stdout = event.stdout.clone().unwrap_or_default();
                    let mut output_layout_constraints = vec![Constraint::Fill(1)];

                    if stdout.contains("assertion `left == right` failed")
                    {
                        let mut expected = String::default();
                        let mut actual = String::default();

                        for line in stdout.lines()
                        {
                            if let Some(left) = line.strip_prefix("  left: ")
                            {
                                expected = left.to_string();
                            }

                            if let Some(right) = line.strip_prefix(" right: ")
                            {
                                actual = right.to_string();
                            }
                        }

                        if !expected.is_empty() && !actual.is_empty()
                        {
                            output_layout_constraints.push(Constraint::Fill(1));
                            let output_layout = super::renderer::build_layout(Direction::Horizontal, output_layout_constraints, main_layout[2]);

                            let expect_result_block = super::renderer::build_block_with_title("Expected".to_string(), EXPECTED_OUTPUT_BLOCK_PADDING, Borders::ALL, background_style, title_style, border_style);
                            let expect_result_paragraph = super::renderer::build_paragraph(expected, Wrap { trim: true }, text_style, expect_result_block);
                            frame.render_widget(expect_result_paragraph, output_layout[0]);

                            let actual_result_block = super::renderer::build_block_with_title("Actual".to_string(), ACTUAL_OUTPUT_BLOCK_PADDING, Borders::ALL, background_style, title_style, border_style);
                            let actual_result_paragraph = super::renderer::build_paragraph(actual, Wrap { trim: true }, text_style, actual_result_block);
                            frame.render_widget(actual_result_paragraph, output_layout[1]);
                        }
                    }

                    else
                    {
                        let mut error = String::default();
                        let mut panic_line_found = false;

                        for error_line in stdout.split("\n")
                        {
                            if panic_line_found
                            {
                                let error_text = format!("{}\n", error_line);
                                error.push_str(&error_text);
                            }

                            if error_line.contains("panicked at")
                            {
                                panic_line_found = true;
                            }
                        }

                        let output_layout = super::renderer::build_layout(Direction::Vertical, output_layout_constraints, main_layout[2]);

                        let output_block = super::renderer::build_block_with_title("Output".to_string(), OUTPUT_BLOCK_PADDING, Borders::ALL, background_style, text_style, border_style);
                        let output_paragraph = super::renderer::build_paragraph(error, Wrap { trim: true }, text_style, output_block);
                        frame.render_widget(output_paragraph, output_layout[0]);
                    }
                }
            }
        })?;

        Ok(())
    }
}