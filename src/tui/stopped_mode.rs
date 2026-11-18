use crate::{
    schedule::entry::Entry,
    tui::{
        app::App,
        app_mode::{AppMode, ContinuationResult},
        blit::blit_by_scale,
        glyphs::chars::NUMS,
        paused_mode::PausedMode,
    },
};
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Stylize},
    symbols::border,
    text::{Line, Text},
    widgets::{Block, Paragraph, Widget},
};
use std::time::Instant;

#[derive(Debug)]
pub struct StoppedMode {
    _start: Instant,
    schedule: Option<Entry>,
}

impl StoppedMode {
    pub fn new(schedule: Option<Entry>) -> Self {
        Self {
            _start: Instant::now(),
            schedule,
        }
    }

    fn instructions(&self) -> Line<'_> {
        Line::from(vec![
            " Start ".into(),
            "<Space>".blue().bold(),
            " Quit ".into(),
            "<q> ".blue().bold(),
        ])
    }

    pub(crate) fn handle_key_event(&mut self, key_event: KeyEvent) -> ContinuationResult {
        match key_event.code {
            KeyCode::Char('q') | KeyCode::Char('Q') => ContinuationResult::Exit,
            KeyCode::Char(' ') => ContinuationResult::ChangeMode {
                updated: App::Paused {
                    mode: PausedMode::new(self.schedule.take()),
                },
            },
            _ => ContinuationResult::Noop,
        }
    }
}

impl AppMode for StoppedMode {
    fn render(&self, area: Rect, buf: &mut Buffer) {
        let scale: u16 = 3;
        let mut left = 4;
        for num in NUMS {
            blit_by_scale(num, (left, 4), scale, area, buf, Color::LightCyan);
            left += scale * (1 + num[0].chars().count() as u16);
        }

        let title = Line::from(" Stopped ".bold());
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
