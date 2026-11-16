use std::io;

use test_project::tui::app::App;

fn main() -> io::Result<()> {
    ratatui::run(|terminal| App::default().run(terminal))
}
