use std::process::exit;

use clap::{crate_version, Parser};
use humantime;

/// Print a time duration in an easily readable form
#[derive(Parser)]
#[clap(version=crate_version!())]
struct Opts {
    /// A time duration with a unit (e.g. '352s').
    ///
    /// Acceptable units are: ns, us, ms, sec, min, hours, days, weeks,
    /// months, years (and a few variations and abbreviations). You can use
    /// multiple units if required (e.g. '118 minutes 227 seconds').
    duration: String,
}

fn main() {
    let opts: Opts = Opts::parse();

    match humantime::parse_duration(&opts.duration) {
        Ok(duration) => {
            println!("{}", humantime::format_duration(duration))
        }
        Err(err) => {
            eprintln!("error: {}", err);
            exit(1)
        }
    }
}
