use clap::{crate_version, Parser};
mod time_interval;

#[derive(Parser)]
#[clap(version=crate_version!(), author="Andrew Dawson <andrew.dawson@ecmwf.int>")]
#[clap(about = "Convert a time interval in seconds to days, hours, minutes, and seconds")]
struct Opts {
    #[clap(about = "A time interval in seconds")]
    seconds: u64,
}

fn main() {
    let opts: Opts = Opts::parse();

    let time_interval = time_interval::TimeInterval::from_seconds(opts.seconds);
    println!("{}", time_interval.format());
}
