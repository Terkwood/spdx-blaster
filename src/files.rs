// SPDX-License-Identifier: MIT
use log::info;
use memmap::{MmapMut, MmapOptions};
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
    let license_line: Vec<u8> = format!("{}{}\n", comment, license).bytes().collect();

    let file = OpenOptions::new().read(true).write(true).open(path)?;
    let file_len: usize = metadata(path).expect("file metadata").len() as usize;

    let license_line_len = license_line.len();

    let total_len = file_len + license_line_len;

    file.set_len(total_len as u64).expect("set file len");

    let mut map = unsafe { MmapOptions::new().map_mut(&file)? };

    let opts = &AlterOpts {
        total_len,
        license_line_len,
        license_line,
    };
    if file_contains_shebang(&map) {
        alter_with_shebang(&mut map, opts)
    } else {
        alter_basic(&mut map, opts)
    }

    map.flush()
}

const SHEBANG_LEN: usize = 2;
const SHEBANG: &str = "#!";
fn file_contains_shebang(map: &MmapMut) -> bool {
    let first_few_chars = std::cmp::min(map.len(), SHEBANG_LEN);
    std::str::from_utf8(&map[..first_few_chars]).unwrap_or_default() == SHEBANG
}

fn alter_with_shebang(map: &mut MmapMut, opts: &AlterOpts) {
    let first_newline_pos = map.iter().position(|c| c == &b'\n').unwrap_or(map.len());
    info!("newline position is {}", first_newline_pos);

    map.rotate_left(first_newline_pos);

    map[(opts.total_len - opts.license_line_len - first_newline_pos)
        ..(opts.total_len - first_newline_pos)]
        .copy_from_slice(&opts.license_line);

    map.rotate_right(opts.license_line_len)
}

fn alter_basic(map: &mut MmapMut, opts: &AlterOpts) {
    map[(opts.total_len - opts.license_line_len)..].copy_from_slice(&opts.license_line);

    map.rotate_right(opts.license_line_len)
}

struct AlterOpts {
    total_len: usize,
    license_line_len: usize,
    license_line: Vec<u8>,
}
/// Path::is_dir() is not guaranteed to be intuitively correct for "." and ".."
/// See: https://github.com/rust-lang/rust/issues/45302
/// Attribution: https://github.com/sharkdp/fd/blob/master/src/filesystem.rs
pub fn is_dir(path: &Path) -> bool {
    path.is_dir() && (path.file_name().is_some() || path.canonicalize().is_ok())
}
