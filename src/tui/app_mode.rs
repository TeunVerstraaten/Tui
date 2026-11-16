use std::io;

use ratatui::{buffer::Buffer, layout::Rect};

pub enum HandleEventResult {
    Ok,
    Exit,
}

pub trait AppMode {
    fn render(&self, area: Rect, buf: &mut Buffer);
    fn handle_events(&mut self) -> io::Result<HandleEventResult>;
}
