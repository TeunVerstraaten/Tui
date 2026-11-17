use ratatui::text::Line;
use serde::{Deserialize, Serialize};
use std::time::Duration;
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize)]
pub enum Entry {
    Simple {
        name: String,
        duration: Duration,
        #[serde(skip_serializing)]
        #[serde(skip_deserializing)]
        uuid: Uuid,
    },
    Sequence {
        entries: Vec<Entry>,
    },
    Repeat {
        entry: Box<Entry>,
        repeat: u16,
    },
}

pub fn simple(name: impl Into<String>, duration: Duration) -> Entry {
    Entry::Simple {
        name: name.into(),
        duration,
        uuid: Uuid::new_v4(),
    }
}

pub fn repeated(entry: Entry, repeat: u16) -> Entry {
    Entry::Repeat {
        entry: Box::new(entry),
        repeat,
    }
}

pub fn sequence(entries: Vec<Entry>) -> Entry {
    Entry::Sequence { entries }
}

fn tree_print_recursive(
    first_prefix: &str,
    second_prefix: &str,
    entry: &Entry,
    lines: &mut Vec<Line<'static>>,
) {
    match entry {
        Entry::Simple { name, duration, .. } => {
            let line = format!("{} {} ({}s)", first_prefix, name, duration.as_secs(),);
            lines.push(Line::from(line));
        }
        Entry::Sequence { entries } => {
            let mut prefix = first_prefix;
            let n = entries.len();

            for (index, entry) in entries.iter().enumerate() {
                let (p1, p2) = match (index, n) {
                    (_, 1) => (
                        &format!("{}────", prefix),
                        &format!("{}    ", second_prefix),
                    ),
                    (0, _) => (&format!("{}┬───", prefix), &format!("{}│   ", prefix)),
                    _ if index + 1 == n => (&format!("{}└───", prefix), &format!("{}    ", prefix)),
                    _ => (&format!("{}├───", prefix), &format!("{}│   ", prefix)),
                };

                tree_print_recursive(p1, p2, entry, lines);
                prefix = second_prefix;
            }
        }
        Entry::Repeat { entry, repeat } => {
            tree_print_recursive(
                &format!("{}{:─<4}", first_prefix, repeat.to_string() + "x "),
                &format!("{}    ", second_prefix),
                entry,
                lines,
            );
        }
    };
}

pub fn tree_print(entry: &Entry) -> Vec<Line<'static>> {
    let mut lines = vec![];
    tree_print_recursive("", "", entry, &mut lines);
    lines
}

pub fn test_schedule() -> Entry {
    repeated(
        sequence(vec![
            sequence(vec![simple("work", Duration::from_secs(40))]),
            sequence(vec![sequence(vec![
                simple("work", Duration::from_secs(40)),
                simple("worsdsdasak", Duration::from_secs(40)),
                simple("Hallo", Duration::from_secs(20)),
            ])]),
            sequence(vec![
                simple("ads", Duration::from_secs(12)),
                repeated(
                    sequence(vec![
                        simple("work", Duration::from_secs(40)),
                        simple("worsdsdasak", Duration::from_secs(40)),
                        simple("Hallo", Duration::from_secs(20)),
                    ]),
                    4,
                ),
            ]),
            sequence(vec![]),
            simple("pause", Duration::from_secs(20)),
            sequence(vec![
                simple("work", Duration::from_secs(40)),
                simple("Hallo", Duration::from_secs(20)),
            ]),
            simple("pause", Duration::from_secs(20)),
        ]),
        4,
    )
}
