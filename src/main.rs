use std::process::exit;

use clap::{crate_version, Parser};
use humantime;

#[derive(Parser)]
#[clap(version=crate_version!(), author="Andrew Dawson <andrew.dawson@ecmwf.int>")]
#[clap(about = "Convert a time duration to a human readable form")]
struct Opts {
    #[clap(help = "Time duration with a unit (e.g. '352s', '652minutes')")]
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
