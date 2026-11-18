use test_project::{schedule::entry::{test_schedule, tree_print}, tui::app::App};

fn main() -> anyhow::Result<()> {
    ratatui::run(|terminal| App::default().run(terminal))?;
    let lines = tree_print(&test_schedule());
    for line in lines {
        println!("{}", line);
    }

    Ok(())
}
