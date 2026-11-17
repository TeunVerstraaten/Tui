use std::time::Duration;

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
