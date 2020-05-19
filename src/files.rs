use log::info;
use memmap::MmapOptions;
use std::fs::{metadata, OpenOptions};
use std::io::Error;
use std::path::Path;

use crate::comment::Comment;
use crate::license::License;

/// Prepend the appropriate license to a given file.
/// Note that the unsafe code used here is required
/// by the memory map of the underlying file.
/// If any other process accesses the file, we're going to
/// crash.  Hooray.
pub fn apply_license(path: &Path, license: License, comment: Comment) -> Result<(), Error> {
    let license_line: Vec<u8> = format!("{} {}", comment, license).bytes().collect();

    info!("hi {} with len {} ", license, license_line.len());

    let file = OpenOptions::new().read(true).write(true).open(path)?;
    let file_len: usize = metadata(path).expect("file metadata").len() as usize;

    info!("file len {}", file_len);
    let license_line_len = license_line.len();

    let total_len = file_len + license_line_len;

    file.set_len(total_len as u64).expect("set file len");

    let mut map = unsafe { MmapOptions::new().map_mut(&file)? };

    map[(total_len - license_line_len)..].copy_from_slice(&license_line);

    map.rotate_right(license_line_len);

    map.flush()
}

/// Path::is_dir() is not guaranteed to be intuitively correct for "." and ".."
/// See: https://github.com/rust-lang/rust/issues/45302
/// Attribution: https://github.com/sharkdp/fd/blob/master/src/filesystem.rs
pub fn is_dir(path: &Path) -> bool {
    path.is_dir() && (path.file_name().is_some() || path.canonicalize().is_ok())
}
