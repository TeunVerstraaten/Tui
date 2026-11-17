use std::time::Duration;

use test_project::schedule::entry::*;

fn main() -> anyhow::Result<()> {
    let schedule = sequence(vec![
        sequence(vec![simple("work", Duration::from_secs(40))]),
        sequence(vec![
            simple("work", Duration::from_secs(40)),
            simple("worsdsdasak", Duration::from_secs(40)),
            simple("Hallo", Duration::from_secs(20)),
        ]),
        simple("pause", Duration::from_secs(20)),
        sequence(vec![
            simple("work", Duration::from_secs(40)),
            simple("Hallo", Duration::from_secs(20)),
        ]),
        simple("pause", Duration::from_secs(20)),
    ]);

    // let schedule = repeated(
    //     sequence(vec![
    //         simple("work", Duration::from_secs(40)),
    //         simple("pause", Duration::from_secs(20)),
    //     ]),
    //     3,
    // );
    //
    tree_print(&schedule);

    // ratatui::run(|terminal| App::default().run(terminal))?;
    Ok(())
}
