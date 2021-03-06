// SPDX-License-Identifier: MIT
extern crate clap;
extern crate env_logger;
extern crate log;
extern crate memmap;

pub mod blaster;
mod comment;
mod dialect;
mod files;
mod ids;
mod license;

use clap::{App, Arg};
use log::trace;
use std::path::Path;

use crate::blaster::Opts;
use crate::license::License;

const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Writes [SPDX license IDs](https://spdx.dev/ids/) into your source
/// files.
fn main() {
    env_logger::init();
    trace!("🔢 {}", VERSION);
    let default_license_id = License::MIT.id();
    let args = App::new(NAME)
        .version(VERSION)
        .author("terkwood <38859656+Terkwood@users.noreply.github.com>")
        .about("Writes SPDX license IDs in your (so far, rust) source files.")
        .arg(
            Arg::with_name("license")
                .short("l")
                .long("license")
                .help("Specify which license you wish to apply. Use the official SPDX license ID, e.g.  GPL-2.0-or-later")
                .takes_value(true)
                .default_value(&default_license_id),
        )
        .arg(
            Arg::with_name("display")
                .short("d")
                .long("display")
                .help("Display which files would receive license strings")
                .takes_value(false),
        )
        .arg(
            Arg::with_name("target")
                .help("Sets the target directory (or file) to visit")
                .required(false)
                .default_value(".")
                .takes_value(true),
        )
        .get_matches();

    trace!("Args: {:?}", args);
    blaster::visit(
        Path::new(args.value_of("target").expect("arg target")),
        Opts {
            display: args.is_present("display"),
            license: License::from(args.value_of("license").unwrap_or_default()),
        },
    )
    .expect("blasted")
}
