use log::warn;
use std::path::Path;

pub struct Opts {
    pub display: bool,
}

impl Default for Opts {
    fn default() -> Self {
        Opts { display: false }
    }
}

pub fn visit(target: &Path, _opts: Opts) {
    if !is_dir(target) {
        apply_license(target)
    } else {
        warn!("Skipping dir: {:?}", target)
    }
}

/// Path::is_dir() is not guaranteed to be intuitively correct for "." and ".."
/// See: https://github.com/rust-lang/rust/issues/45302
/// Attribution: https://github.com/sharkdp/fd/blob/master/src/filesystem.rs
fn is_dir(path: &Path) -> bool {
    path.is_dir() && (path.file_name().is_some() || path.canonicalize().is_ok())
}

fn apply_license(file: &Path) {
    todo!("no impl")
}
