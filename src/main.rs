// SPDX-License-Identifier: MIT
extern crate clap;
extern crate env_logger;
extern crate log;
extern crate memmap;

pub mod blaster;
pub mod comment;
pub mod dialect;
pub mod files;
mod license;
pub mod source;

use clap::{App, Arg};
use log::trace;
use std::path::Path;

use crate::blaster::Opts;
use crate::license::License;

const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() {
    env_logger::init();
    trace!("🔢 {}", VERSION);
    let args = App::new(NAME)
        .version(VERSION)
        .author("terkwood <38859656+Terkwood@users.noreply.github.com>")
        .about("Writes SPDX license IDs in your (so far, rust) source files.")
        .arg(
            Arg::with_name("license")
                .short("l")
                .long("license")
                .help("Specify which license you wish to apply")
                .takes_value(true)
                .default_value("MIT"),
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
