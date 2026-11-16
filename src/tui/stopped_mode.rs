use crate::tui::{
    app::App,
    app_mode::{AppMode, ContinuationResult},
    glyphs::chars::{NUMS, ONE, TWO, ZERO},
    paused_mode::PausedMode,
};
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use palette::{IntoColor, Okhsv, Srgb};
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
}

impl Default for StoppedMode {
    fn default() -> Self {
        Self {
            _start: Instant::now(),
        }
    }
}

impl StoppedMode {
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
                    mode: PausedMode::default(),
                },
            },
            _ => ContinuationResult::Noop,
        }
    }

    fn blit_by_scale(
        c: &[&str],
        (left, bottom): (u16, u16),
        scale: u16,
        area: Rect,
        buf: &mut Buffer,
    ) {
        let fg = Color::LightMagenta;
        let mut bottom = bottom;
        for row in c.iter() {
            let mut left = left;

            for c in row.chars() {
                match c {
                    ' ' => (),
                    c => {
                        for i_ in left..(left + scale).min(area.width - 1) {
                            for j_ in bottom..(bottom + scale).min(area.height - 1) {
                                buf[(i_, j_)].set_char(c).set_fg(fg);
                            }
                        }
                    }
                }

                left += scale;
            }
            bottom += scale;
        }
    }
}

impl AppMode for StoppedMode {
    fn render(&self, area: Rect, buf: &mut Buffer) {
        let scale: u16 = 2;
        let mut left = 4;

        for i in 0..=9 {
            Self::blit_by_scale(NUMS[i], (left, 4), scale, area, buf);
            left += scale * (1 + NUMS[i][0].chars().count() as u16);
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
