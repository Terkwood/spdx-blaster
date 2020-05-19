// SPDX-License-Identifier: MIT
use log::warn;
use std::io;
use std::path::Path;

use crate::dialect::Dialect;
use crate::files::apply_license;
use crate::files::is_dir;
use crate::license::License;

pub fn visit(target: &Path, opts: Opts) -> Result<(), VisitError> {
    if !is_dir(target) {
        Ok(apply_license(
            target,
            opts.license,
            Dialect::from(target).comment(),
        )?)
    } else {
        warn!("Skipping dir: {:?}", target);
        Ok(())
    }
}

pub struct Opts {
    pub display: bool,
    pub license: License,
}

impl Default for Opts {
    fn default() -> Self {
        Opts {
            display: false,
            license: License::MIT,
        }
    }
}

#[derive(Debug)]
pub enum VisitError {
    IO(io::Error),
}

impl From<io::Error> for VisitError {
    fn from(e: io::Error) -> Self {
        VisitError::IO(e)
    }
}
