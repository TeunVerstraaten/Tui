use std::io;

use ratatui::{buffer::Buffer, layout::Rect};

use crate::tui::app::App;

pub enum ContinuationResult {
    Noop,
    ChangeMode { updated: App },
    Exit,
}

pub trait AppMode {
    fn on_tick(&self) -> ContinuationResult;
    fn render(&self, area: Rect, buf: &mut Buffer);
    fn handle_events(&mut self) -> io::Result<ContinuationResult>;
}
