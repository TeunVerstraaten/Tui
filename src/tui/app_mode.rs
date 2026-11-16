use crate::tui::app::App;
use ratatui::{buffer::Buffer, layout::Rect};
use std::io;

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
