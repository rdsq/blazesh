pub enum GitVerbosityConfig {
    Minimal,
    Remotes,
    All,
}

impl GitVerbosityConfig {
    pub fn detect() -> Self {
        if let Ok(val) = std::env::var("BLAZESH_GIT_VERBOSITY") {
            match &val as &str {
                "minimal" => Self::Minimal,
                "remotes" => Self::Remotes,
                "all" => Self::All,
                val => {
                    eprintln!("blazesh: unknown BLAZESH_GIT_VERBOSITY value: {}", val);
                    Self::Minimal
                },
            }
        } else {
            // default
            Self::Minimal
        }
    }

    pub fn show_remotes(&self) -> bool {
        matches!(self, Self::Remotes | Self::All)
    }

    pub fn show_uncommitted(&self) -> bool {
        matches!(self, Self::All)
    }
}
