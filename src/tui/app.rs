use std::{
    io,
    time::{Duration, Instant},
};

use crossterm::event;
use ratatui::{DefaultTerminal, Frame, buffer::Buffer, layout::Rect, widgets::Widget};

use crate::tui::{
    app_mode::{AppMode, ContinuationResult},
    paused_mode::PausedMode,
    stopped_mode::StoppedMode,
};

#[derive(Debug)]
pub enum App {
    Stopped { mode: StoppedMode },
    Paused { mode: PausedMode },
}

impl Default for App {
    fn default() -> Self {
        App::Stopped {
            mode: StoppedMode::default(),
        }
    }
}

pub struct RunningMode {
    running: bool,
}

impl App {
    fn handle_continuation_result(
        &mut self,
        continuation_result: ContinuationResult,
    ) -> RunningMode {
        match continuation_result {
            ContinuationResult::Noop => (),
            ContinuationResult::ChangeMode { updated } => *self = updated,
            ContinuationResult::Exit => return RunningMode { running: false },
        };
        RunningMode { running: true }
    }

    fn on_tick(&mut self) -> ContinuationResult {
        match self {
            App::Paused { mode } => mode.on_tick(),
            App::Stopped { mode } => mode.on_tick(),
        }
    }

    fn handle_event(&mut self) -> std::io::Result<ContinuationResult> {
        Ok(match self {
            App::Paused { mode } => mode.handle_events()?,
            App::Stopped { mode } => mode.handle_events()?,
        })
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        let tick_rate = Duration::from_millis(10);
        let mut last_tick = Instant::now();
        loop {
            terminal.draw(|frame| self.draw(frame))?;

            let timeout = tick_rate.saturating_sub(last_tick.elapsed());
            let continuation_result = if !event::poll(timeout)? {
                let continuation_result = self.on_tick();
                last_tick = Instant::now();
                self.handle_continuation_result(continuation_result)
            } else {
                let continuation_result = self.handle_event()?;
                self.handle_continuation_result(continuation_result)
            };

            if !continuation_result.running {
                break;
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
            App::Stopped { mode } => mode.render(area, buf),
        }
    }
}
