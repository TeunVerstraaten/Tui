use crate::tui::{
    app::App,
    app_mode::{AppMode, ContinuationResult},
    glyphs::chars::{ONE, TWO},
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

    fn blit_by_ranges(
        c: &[&str],
        x_range: std::ops::Range<u16>,
        y_range: std::ops::Range<u16>,
        area: Rect,
        buf: &mut Buffer,
    ) {
        let c_w = c[0].len() as u16;
        let c_h = c.len() as u16;

        let x_len = x_range.len();
        let y_len = y_range.len();

        let is_pixel_on = |i: u16, j: u16| {
            let i: f64 = (c_w * i) as f64 / ((x_len + 1) as f64);
            let j: f64 = (c_h * j) as f64 / ((y_len + 1) as f64);

            let i = (i.floor() as usize).min(c_w as usize - 1);
            let j = (j.floor() as usize).min(c_h as usize - 1);

            return c[j].as_bytes()[i] != b' ';
        };

        let x = x_range.start;
        let y = y_range.start;

        for i in x_range {
            for j in y_range.clone() {
                let pts = [(i, j), (i + 1, j), (i, j + 1), (i + 1, j + 1)];

                let count = pts
                    .into_iter()
                    .filter(|(i, j)| is_pixel_on(i - x, j - y))
                    .count();
                if (count < 1) {
                    continue;
                }

                let fg = Color::LightYellow;

                buf[(i, j)].set_char('█').set_fg(fg);
            }
        }
    }

    fn blit_by_scale(
        c: &[&str],
        (left, bottom): (u16, u16),
        scale: u16,
        area: Rect,
        buf: &mut Buffer,
    ) {
        let fg = Color::LightYellow;
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
        Self::blit_by_scale(TWO, (4, 4), 2, area, buf);

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
