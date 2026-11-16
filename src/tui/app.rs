use std::io;

use ratatui::{DefaultTerminal, Frame, buffer::Buffer, layout::Rect, widgets::Widget};

use crate::tui::{app_mode::AppMode, paused_mode::PausedMode};

#[derive(Debug)]
pub enum App {
    Paused { mode: PausedMode },
}

impl Default for App {
    fn default() -> Self {
        App::Paused {
            mode: PausedMode::default(),
        }
    }
}

impl App {
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        loop {
            terminal.draw(|frame| self.draw(frame))?;

            let handle_event_result = match self {
                App::Paused { mode } => mode.handle_events()?,
            };

            match handle_event_result {
                super::app_mode::HandleEventResult::Ok => todo!(),
                super::app_mode::HandleEventResult::Exit => break,
            }
        }
        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        match self {
            App::Paused { mode } => mode.render(area, buf),
        }
    }
}

#[cfg(test)]
mod tests {}
