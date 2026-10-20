/*!
 * Handles the core rendering of the TUI.
 */
use std::io;
use std::io::{Error, Stdout};
use std::rc::Rc;
use crossterm::event::KeyCode;
use crossterm::terminal;
use crossterm::terminal::{EnterAlternateScreen, LeaveAlternateScreen};
use ratatui::Terminal;
use ratatui::backend::CrosstermBackend;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Style};
use ratatui::text::Line;
use ratatui::widgets::{Block, Borders, List, ListItem, Padding, Paragraph, Wrap};
use super::super::configuration::core_configuration::Configuration;
use super::super::configuration::theme::Theme;
use super::super::commands::structures::enums::event;
use super::super::commands::structures::enums::event::Event;
use super::super::text::renderer;

/// The padding for the summary block.
pub const SUMMARY_BLOCK_PADDING: Padding = Padding::horizontal(1);

/// Common data for rendering.
#[derive(Debug)]
pub struct RendererData
{
    /// The interface between the renderer and the terminal.
    pub terminal: Terminal<CrosstermBackend<Stdout>>
}

/// The implementation of the [RendererData] to be used with the [Renderer].
impl RendererData
{
    /// Initialises a new instance of the [RendererData].
    /// # Returns
    /// `Result<RendererData, Error>` - Returns the renderer data or an error.
    /// # Errors
    /// Returns an error if the RenderData cannot be created.
    pub fn new() -> Result<Self, Error>
    {
        terminal::enable_raw_mode()?;
        let backend = CrosstermBackend::new(io::stdout());
        let terminal = Terminal::new(backend)?;
        crossterm::execute!(io::stdout(), EnterAlternateScreen)?;

        Ok(Self
        {
            terminal
        })
    }
}

/// Defines the implementing struct as a renderer of command outputs.
pub trait Renderer
{
    /// Retrieve the instance of the renderer data.
    /// # Returns
    /// `&RendererData` - Returns the renderer data.
    fn render_data(&self) -> &RendererData;

    /// Retrieve the mutable instance of the renderer data.
    /// # Returns
    /// `&mut RendererData` - Returns the mutable renderer data.
    fn render_data_mutable(&mut self) -> &mut RendererData;

    /// Renders the UI every update of the command output stream.
    /// # Arguments
    /// * `configuration` - The configuration of Cold Steel.
    /// * `events` - The collection of events being rendered.
    /// * `input_action` - The input action that invoked the render.
    /// # Returns
    /// `Result<(), Error>` - Returns the result of the render or an error.
    /// # Errors
    /// Returns an error if the frame cannot be drawn.
    fn render(&mut self, configuration: &Configuration, events: &[Event], input_action: KeyCode) -> Result<(), Error>;

    /// Cleans up the renderer and resets the terminal.
    /// # Returns
    /// `Result<(), Error>` - Returns the result of the exit or an error.
    /// # Errors
    /// Returns an error if the Renderer cannot be cleanly exited.
    fn exit(&mut self) -> Result<(), Error>
    {
        terminal::disable_raw_mode()?;
        let backend = self.render_data_mutable().terminal.backend_mut();
        crossterm::execute!(backend, LeaveAlternateScreen)?;
        self.render_data_mutable().terminal.show_cursor()?;

        Ok(())
    }
}

/// Build the style from a colour.
/// # Arguments
/// * `background` - The background colour to use for the style.
/// * `colour` - The colour to use for the style.
/// # Returns
/// `Style` - Returns the style.
pub fn build_style(background: Color, colour: Color) -> Style
{
    Style::default().bg(background).fg(colour)
}

/// Build the background style from a theme.
/// # Arguments
/// * `theme` - The theme to use for the style.
/// # Returns
/// `Style` - Returns the background style.
pub fn build_background_style(theme: Option<&Theme>) -> Style
{
    let background = theme.map(|theme| theme.background).unwrap_or(Color::Reset);

    Style::default().bg(background)
}

/// Build the text style from a theme.
/// # Arguments
/// * `theme` - The theme to use for the style.
/// # Returns
/// `Style` - Returns the text style.
pub fn build_text_style(theme: Option<&Theme>) -> Style
{
    let background = theme.map(|theme| theme.background).unwrap_or(Color::Reset);
    let colour = theme.map(|theme| theme.text).unwrap_or(Color::Reset);

    build_style(background, colour)
}

/// Build the title style from a theme.
/// # Arguments
/// * `theme` - The theme to use for the style.
/// # Returns
/// `Style` - Returns the title style.
pub fn build_title_style(theme: Option<&Theme>) -> Style
{
    let background = theme.map(|theme| theme.background).unwrap_or(Color::Reset);
    let colour = theme.map(|theme| theme.title).unwrap_or(Color::Reset);

    build_style(background, colour)
}

/// Build the passed style from a theme.
/// # Arguments
/// * `theme` - The theme to use for the style.
/// # Returns
/// `Style` - Returns the passed style.
pub fn build_passed_style(theme: Option<&Theme>) -> Style
{
    let background = theme.map(|theme| theme.background).unwrap_or(Color::Reset);
    let colour = theme.map(|theme| theme.passed).unwrap_or(Color::Reset);

    build_style(background, colour)
}

/// Build the failed style from a theme.
/// # Arguments
/// * `theme` - The theme to use for the style.
/// # Returns
/// `Style` - Returns the failed style.
pub fn build_failed_style(theme: Option<&Theme>) -> Style
{
    let background = theme.map(|theme| theme.background).unwrap_or(Color::Reset);
    let colour = theme.map(|theme| theme.failed).unwrap_or(Color::Reset);

    build_style(background, colour)
}

/// Build the highlight style from a theme.
/// # Arguments
/// * `theme` - The theme to use for the style.
/// # Returns
/// `Style` - Returns the highlight style.
pub fn build_highlight_style(theme: Option<&Theme>) -> Style
{
    let background = theme.map(|theme| theme.background).unwrap_or(Color::Reset);
    let colour = theme.map(|theme| theme.highlight).unwrap_or(Color::Reset);

    build_style(background, colour)
}

/// Build the border style from a theme.
/// # Arguments
/// * `theme` - The theme to use for the style.
/// # Returns
/// `Style` - Returns the border style.
pub fn build_border_style(theme: Option<&Theme>) -> Style
{
    let background = theme.map(|theme| theme.background).unwrap_or(Color::Reset);
    let colour = theme.map(|theme| theme.border).unwrap_or(Color::Reset);

    build_style(background, colour)
}

/// Build the layout for the interface.
/// # Arguments
/// * `direction` - The direction of the layout.
/// * `constraints` - The constraints of the layout.
/// * `area` - The area to define the layout in.
/// # Returns
/// `Rc<[Rect]>` - Returns the collection of areas.
pub fn build_layout<I>(direction: Direction, constraints: I, area: Rect) -> Rc<[Rect]>
where
    I: IntoIterator,
    I::Item: Into<Constraint>
{
    Layout::default()
        .direction(direction)
        .constraints(constraints)
        .split(area)
}

/// Build a block.
/// # Arguments
/// * `padding` - The padding of the block.
/// * `borders` - The borders of the block.
/// * `background_style` - The style of the background of the block.
/// * `title_style` - The style of the title(s) of the block.
/// * `border_style` - The style of the border of the block.
/// # Returns
/// `Block` - Returns the block.
pub fn build_block<'a>(padding: Padding, borders: Borders, background_style: Style, title_style: Style, border_style: Style) -> Block<'a>
{
    Block::default()
        .padding(padding)
        .borders(borders)
        .style(background_style)
        .title_style(title_style)
        .border_style(border_style)
}

/// Build a block with a title.
/// # Arguments
/// * `title` - The title of the block.
/// * `padding` - The padding of the block.
/// * `borders` - The borders of the block.
/// * `background_style` - The style of the background of the block.
/// * `title_style` - The style of the title(s) of the block.
/// * `border_style` - The style of the border of the block.
/// # Returns
/// `Block` - Returns the block.
pub fn build_block_with_title<'a>(title: String, padding: Padding, borders: Borders, background_style: Style, title_style: Style, border_style: Style) -> Block<'a>
{
    Block::default()
        .title(Line::from(title).left_aligned())
        .padding(padding)
        .borders(borders)
        .style(background_style)
        .title_style(title_style)
        .border_style(border_style)
}

/// Build a paragraph.
/// # Arguments
/// * `text` - The text of the paragraph.
/// * `wrap` - The wrap of the paragraph.
/// * `style` - The style of the paragraph.
/// * `block` - The block of the paragraph.
/// # Returns
/// `Paragraph` - Returns the paragraph.
pub fn build_paragraph(text: String, wrap: Wrap, style: Style, block: Block<'_>) -> Paragraph<'_>
{
    Paragraph::new(text)
        .wrap(wrap)
        .style(style)
        .block(block)
}

/// Build the summary block for a suite.
/// # Arguments
/// * `right_title` - The title to display on the right alignment, in the summary.
/// * `background_style` - The style of the background of the summary block.
/// * `title_style` - The style of the title(s) of the summary block.
/// * `border_style` - The style of the border of the summary block.
/// # Returns
/// `Block` - Returns the summary block.
pub fn build_summary_block<'a>(right_title: String, background_style: Style, title_style: Style, border_style: Style) -> Block<'a>
{
    Block::default()
        .title(Line::from("Suite Summary").left_aligned())
        .title(Line::from(right_title).right_aligned())
        .title_bottom(Line::from("(Exit) [ESC]").right_aligned())
        .padding(SUMMARY_BLOCK_PADDING)
        .borders(Borders::ALL)
        .style(background_style)
        .title_style(title_style)
        .border_style(border_style)
}

/// Build the block for the list of events.
/// # Arguments
/// * `left_title` - The title to display on the left alignment, in the list block.
/// * `right_title` - The title to display on the right alignment, in the list block.
/// * `bottom_right_title` - The title to display on the bottom right alignment, in the list block.
/// * `event_list` - The list of events to index for the centre title.
/// * `event_index` - The index of the current event for the centre title.
/// * `background_style` - The style of the background of the list block.
/// * `title_style` - The style of the title(s) of the list block.
/// * `border_style` - The style of the border of the list block.
/// # Returns
/// `Block` - Returns the list block.
pub fn build_list_block<'a>(left_title: String, right_title: String, bottom_right_title: String, event_list: &[&Event], event_index: usize, background_style: Style, title_style: Style, border_style: Style) -> Block<'a>
{
    let mut list_block = Block::default()
        .title(Line::from(left_title).left_aligned())
        .title(Line::from(right_title).right_aligned())
        .title_bottom(Line::from(bottom_right_title).right_aligned())
        .borders(Borders::ALL)
        .style(background_style)
        .title_style(title_style)
        .border_style(border_style);

    if event_list.len() > 0
    {
        let event = event_list[event_index];
        let event_name= event::determine_event_name(event);
        let list_block_title = format!("{}", event_name.split("::").last().unwrap_or_default());
        list_block = list_block.title(Line::from(list_block_title).centered());
    }

    list_block
}

/// Build the list of events.
/// # Arguments
/// * `items` - The items to display in the list.
/// * `block` - The block to use for the list.
/// * `highlight_style` - The highlight style of the list.
/// # Returns
/// `List` - Returns the list.
pub fn build_list<'a>(items: Vec<ListItem<'a>>, block: Block<'a>, highlight_style: Style) -> List<'a>
{
    List::new(items)
        .highlight_style(highlight_style)
        .highlight_symbol(renderer::render_text("", "»", "»"))
        .block(block)
}

/// Wraps the index to the maximum index.
/// # Arguments
/// * `index` - The index to wrap.
/// * `maximum_index` - The maximum index to wrap by.
/// # Returns
/// `usize` - The wrapped index.
pub fn wrap_index(index: usize, maximum_index: usize) -> usize
{
    if maximum_index != 0
    {
        return (index + maximum_index).rem_euclid(maximum_index)
    }

    0
}

/// Tests for the [wrap_index] function.
#[cfg(test)]
mod tests
{
    use super::*;

    /// Tests the [wrap_index] function to assert wrapping is correct.
    #[test]
    fn test_wrap_index_normal()
    {
        assert_eq!(wrap_index(5, 10), 5);
    }

    /// Tests the [wrap_index] function to assert wrapping is correct when the index is greater than the maximum index.
    #[test]
    fn test_wrap_index_overflow()
    {
        assert_eq!(wrap_index(15, 10), 5);
    }

    /// Tests the [wrap_index] function to assert wrapping is correct when the maximum index is 0.
    #[test]
    fn test_wrap_index_zero_max()
    {
        assert_eq!(wrap_index(5, 0), 0);
    }
}