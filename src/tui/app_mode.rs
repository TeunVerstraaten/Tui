#[derive(Debug, Default)]
pub enum AppMode {
    Running,
    #[default]
    Paused,
}

impl AppMode {
    pub(crate) fn instructions(&self) -> Vec<(&str, &str)> {
        match self {
            Self::Running => vec![],
            Self::Paused => vec![("Play", "Space"), ("Reload", "r")],
        }
    }
}
