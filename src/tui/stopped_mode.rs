use crate::tui::{
    app::App,
    app_mode::{AppMode, ContinuationResult},
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
}

fn color_from_oklab(hue: f32, saturation: f32, value: f32) -> Color {
    let color: Srgb = Okhsv::new(hue, saturation, value).into_color();
    let color = color.into_format();
    Color::Rgb(color.red, color.green, color.blue)
}

impl AppMode for StoppedMode {
    fn render(&self, area: Rect, buf: &mut Buffer) {
        for (yi, y) in (area.top()..area.bottom()).enumerate() {
            let value = f32::from(area.height) - yi as f32;
            let value_fg = value / f32::from(area.height);
            for (xi, x) in (area.left()..area.right()).enumerate() {
                let hue = xi as f32 * 360.0 / f32::from(area.width);
                let fg = color_from_oklab(hue, Okhsv::max_saturation(), value_fg);
                buf[(x, y)].set_char('█').set_fg(fg);
            }
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
