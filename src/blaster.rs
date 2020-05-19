use std::path::Path;

pub struct Opts {
    pub write: bool,
}

impl Default for Opts {
    fn default() -> Self {
        Opts { write: true }
    }
}

pub fn visit(_target: &Path, _opts: Opts) {}
