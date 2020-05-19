extern crate clap;
extern crate env_logger;
extern crate log;

pub mod blaster;
pub mod comment;
pub mod dialect;
pub mod id;
pub mod source;

use clap::{App, Arg};
use log::info;
use std::path::Path;

use crate::blaster::Opts;

const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() {
    env_logger::init();
    info!("🔢 {}", VERSION);
    let args = App::new(NAME)
        .version(VERSION)
        .author("terkwood <38859656+Terkwood@users.noreply.github.com>")
        .about("Writes SPDX license IDs in your (so far, rust) source files.")
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

    info!("Args: {:?}", args);
    blaster::visit(
        Path::new(args.value_of("target").expect("arg target")),
        Opts {
            display: args.is_present("display"),
        },
    )
}
