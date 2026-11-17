use std::time::Duration;

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub enum BasicEntry {
    Simple { name: String, duration: Duration },
    Sequence { entries: Vec<BasicEntry> },
}

#[derive(Debug, Deserialize, Serialize)]
pub enum Entry {
    Basic { entry: BasicEntry },
    Repeated { entry: BasicEntry, repeat: u16 },
}

pub fn simple(name: impl Into<String>, duration: Duration) -> BasicEntry {
    BasicEntry::Simple {
        name: name.into(),
        duration,
    }
}

pub fn sequence(entries: Vec<BasicEntry>) -> BasicEntry {
    BasicEntry::Sequence { entries }
}

pub fn basic(entry: BasicEntry) -> Entry {
    Entry::Basic { entry }
}

pub fn repeated(entry: BasicEntry, repeat: u16) -> Entry {
    Entry::Repeated { entry, repeat }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let schedule = repeated(
            sequence(vec![
                simple("work", Duration::from_secs(40)),
                simple("pause", Duration::from_secs(40)),
            ]),
            3,
        );

        println!("{:?}", serde_json::to_string(schedule).unwrap());
    }
}
