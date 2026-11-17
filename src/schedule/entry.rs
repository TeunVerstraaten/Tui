use std::{collections::HashSet, time::Duration};

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub enum Entry {
    Simple { name: String, duration: Duration },
    Sequence { entries: Vec<Entry> },
    Repeat { entry: Box<Entry>, repeat: u16 },
}

pub fn simple(name: impl Into<String>, duration: Duration) -> Entry {
    Entry::Simple {
        name: name.into(),
        duration,
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

fn tree_print_recursive(first_prefix: &str, second_prefix: &str, entry: &Entry) {
    match entry {
        Entry::Simple { name, duration } => {
            println!("{} {} ({}s)", first_prefix, name, duration.as_secs());
        }
        Entry::Sequence { entries } => {
            let mut prefix = first_prefix;
            let n = entries.len();

            for (index, entry) in entries.iter().enumerate() {
                let (p1, p2) = match (index, n) {
                    (_, 1) => (&format!("{}────", prefix), &format!("{}│   ", prefix)),
                    (0, _) => (&format!("{}┬───", prefix), &format!("{}│   ", prefix)),
                    _ if index + 1 == n => (&format!("{}└───", prefix), &format!("{}    ", prefix)),
                    _ => (&format!("{}├───", prefix), &format!("{}│   ", prefix)),
                };

                tree_print_recursive(p1, p2, entry);
                prefix = second_prefix;
            }
        }
        Entry::Repeat {
            entry: _,
            repeat: _,
        } => {
            println!("{:.<5}", 12);
            todo!();
            // for _ in 0..*repeat {
            //     tree_print_recursive(depth + 1, entry);
            // }
        }
    };
}

// fn tree_print_recursive(depth: usize, bars: &mut HashSet<usize>, entry: &Entry) {
//     match entry {
//         Entry::Simple { name, duration } => {
//             let line = " ".repeat(depth * 4)
//
//             // println!("{} {} ({}s)", first_prefix, name, duration.as_secs());
//         }
//         Entry::Sequence { entries } => {
//             for entry in entries {
//                 bars.insert(depth);
//             }
//         }
//         Entry::Repeat {
//             entry: _,
//             repeat: _,
//         } => {
//             println!("{:.<5}", 12);
//             todo!();
//             // for _ in 0..*repeat {
//             //     tree_print_recursive(depth + 1, entry);
//             // }
//         }
//     };
// }

pub fn tree_print(entry: &Entry) {
    tree_print_recursive("", "", entry);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let schedule = repeated(
            sequence(vec![
                simple("work", Duration::from_secs(40)),
                simple("pause", Duration::from_secs(20)),
            ]),
            3,
        );

        println!("{}", serde_json::to_string(&schedule).unwrap());
        assert_eq!(1, 2);
    }
}
