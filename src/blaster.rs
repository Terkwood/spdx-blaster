use std::path::Path;

pub struct Opts {
    pub display: bool,
}

impl Default for Opts {
    fn default() -> Self {
        Opts { display: false }
    }
}

pub fn visit(_target: &Path, _opts: Opts) {}
