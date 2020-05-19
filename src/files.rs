use log::info;
use memmap::MmapOptions;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::{Error, Write};
use std::path::Path;

use crate::license::License;

/// Prepend the appropriate license to a given file.
/// Note that the unsafe code used here is required
/// by the memory map of the underlying file.
/// If any other process accesses the file, we're going to
/// crash.  Hooray.
pub fn apply_license(path: &Path, license: License) -> Result<(), Error> {
    let license_line: Vec<u8> = license.to_string().bytes().collect();

    info!("hi {} ", license);

    let file = OpenOptions::new().read(true).write(true).open(path)?;
    let file_len: usize = todo!();

    let license_line_len = license_line.len();
    let total_len = file_len + license_line_len;

    let mut map = unsafe { MmapOptions::new().len(todo!()).map_mut(&file)? };

    map.rotate_right(license_line_len);

    map[..license_line_len].copy_from_slice(&license_line);

    map.flush()
}

/// Path::is_dir() is not guaranteed to be intuitively correct for "." and ".."
/// See: https://github.com/rust-lang/rust/issues/45302
/// Attribution: https://github.com/sharkdp/fd/blob/master/src/filesystem.rs
pub fn is_dir(path: &Path) -> bool {
    path.is_dir() && (path.file_name().is_some() || path.canonicalize().is_ok())
}
