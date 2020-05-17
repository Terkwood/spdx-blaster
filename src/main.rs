extern crate clap;
extern crate env_logger;
extern crate log;

pub mod comment;
pub mod dialect;
pub mod source;

use clap::{App, Arg};
use log::info;
const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");
fn main() {
    env_logger::init();
    let args = App::new(NAME)
        .version(VERSION)
        .author("terkwood <38859656+Terkwood@users.noreply.github.com>")
        .about("Writes SPDX license IDs in your (so far, rust) source files.")
        .arg(
            Arg::with_name("write")
                .short("w")
                .long("write")
                .help("Update all relevant files with license string. If this option is ommitted, you'll simply see output")
                .takes_value(false),
        )
        .arg(
            Arg::with_name("target")
                .help("Sets the target directory to visit")
                .required(false)
                .default_value(".")
                .takes_value( true),
        )
        .get_matches();

    info!("Args: {:?}", args);
}
