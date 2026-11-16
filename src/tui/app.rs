use std::{
    io,
    time::{Duration, Instant},
};

use crossterm::event;
use ratatui::{DefaultTerminal, Frame, buffer::Buffer, layout::Rect, widgets::Widget};

use crate::tui::{
    app_mode::{AppMode, ContinuationResult},
    paused_mode::PausedMode,
};

#[derive(Debug)]
pub enum App {
    Paused { mode: PausedMode },
}

impl Default for App {
    fn default() -> Self {
        App::Paused {
            mode: PausedMode::new(),
        }
    }
}

impl App {
    fn on_tick(&self) -> ContinuationResult {
        ContinuationResult::Ok
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        let tick_rate = Duration::from_millis(10);
        let mut last_tick = Instant::now();
        loop {
            terminal.draw(|frame| self.draw(frame))?;

            let timeout = tick_rate.saturating_sub(last_tick.elapsed());
            if !event::poll(timeout)? {
                let tick_result = self.on_tick();
                match tick_result {
                    ContinuationResult::Ok => continue,
                    ContinuationResult::Exit => break,
                }

                last_tick = Instant::now();
                continue;
            }
            let event_result = match self {
                App::Paused { mode } => mode.handle_events()?,
            };

            match event_result {
                ContinuationResult::Ok => continue,
                ContinuationResult::Exit => break,
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
