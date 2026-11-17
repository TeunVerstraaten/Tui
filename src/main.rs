
use test_project::tui::app::App;

fn main() -> anyhow::Result<()> {
    ratatui::run(|terminal| App::default().run(terminal))?;
    Ok(())
}
