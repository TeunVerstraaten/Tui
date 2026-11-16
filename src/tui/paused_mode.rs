use crate::tui::{
    app::App,
    app_mode::{AppMode, ContinuationResult},
    stopped_mode::StoppedMode,
};
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Stylize,
    symbols::border,
    text::{Line, Text},
    widgets::{Block, Paragraph, Widget},
};
use std::time::Instant;

#[derive(Debug)]
pub struct PausedMode {
    _start: Instant,
}

impl Default for PausedMode {
    fn default() -> Self {
        Self {
            _start: Instant::now(),
        }
    }
}

impl PausedMode {
    fn instructions(&self) -> Line<'_> {
        Line::from(vec![
            " Stop ".into(),
            "<Esc>".blue().bold(),
            " Quit ".into(),
            "<q> ".blue().bold(),
        ])
    }

    pub(crate) fn handle_key_event(&mut self, key_event: KeyEvent) -> ContinuationResult {
        match key_event.code {
            KeyCode::Char('q') | KeyCode::Char('Q') => ContinuationResult::Exit,
            KeyCode::Esc => ContinuationResult::ChangeMode {
                updated: App::Stopped {
                    mode: StoppedMode::default(),
                },
            },
            _ => ContinuationResult::Noop,
        }
    }
}

impl AppMode for PausedMode {
    fn render(&self, area: Rect, buf: &mut Buffer) {
        let title = Line::from(" Paused ".bold());

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

    fn handle_events(&mut self) -> std::io::Result<ContinuationResult> {
        Ok(match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event)
            }
            _ => ContinuationResult::Noop,
        })
    }

    fn on_tick(&self) -> ContinuationResult {
        ContinuationResult::Noop
    }
}
