use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Stylize,
    symbols::border,
    text::{Line, Text},
    widgets::{Block, Paragraph, Widget},
};

use crate::tui::app_mode::{AppMode, HandleEventResult};

#[derive(Debug, Default)]
pub struct PausedMode {}

impl PausedMode {
    fn instructions(&self) -> Line<'_> {
        let instructions = Line::from(vec![
            " Decrement ".into(),
            "<Left>".blue().bold(),
            " Increment ".into(),
            "<Right>".blue().bold(),
            " Quit ".into(),
            "<Q> ".blue().bold(),
        ]);
        Line::from_iter(instructions)
    }

    pub(crate) fn handle_key_event(&mut self, key_event: KeyEvent) -> HandleEventResult {
        match key_event.code {
            KeyCode::Char('q') => HandleEventResult::Exit,
            _ => HandleEventResult::Ok,
        }
    }
}

impl AppMode for PausedMode {
    fn render(&self, area: Rect, buf: &mut Buffer) {
        let title = Line::from(" Counter App Tutorial ".bold());

        let instructions = self.instructions();
        let block = Block::bordered()
            .title(title.centered())
            .title_bottom(instructions.centered())
            .border_set(border::THICK);

        let counter_text = Text::from(vec![Line::from(vec!["Value: ".into()])]);

        Paragraph::new(counter_text)
            .centered()
            .block(block)
            .render(area, buf);
    }

    fn handle_events(&mut self) -> std::io::Result<HandleEventResult> {
        let handle_event_return = match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event)
            }
            _ => HandleEventResult::Ok,
        };
        Ok(handle_event_return)
    }
}
