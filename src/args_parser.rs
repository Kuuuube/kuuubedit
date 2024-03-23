use clap::Parser;
use core::str::FromStr;

/// Powerful headless text editor for processing enormous files
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Enables the undo command and --no-buf
    #[arg(short, long)]
    pub undo: bool,

    /// Sets file buffer size
    #[arg(short, long, default_value_t = 100000000, value_parser = buffer_size_check)]
    pub buffer: u64,

    /// Disables file buffering
    #[arg(long)]
    pub no_buf: bool,

    /// Path to file to open
    #[arg(value_name = "FILEPATH")]
    pub file: String,
}

fn buffer_size_check(s: &str) -> Result<u64, String> {
    number_range(s, 1024, u64::MAX)
}

//https://github.com/newAM/clap-num/blob/25b0c5d1a58058b1dd82092155f55c198793d6ed/src/lib.rs#L24-L114
pub fn number_range<T: Ord + PartialOrd + std::fmt::Display>(s: &str, min: T, max: T, ) -> Result<T, String> where T: FromStr, <T as FromStr>::Err: std::fmt::Display {
    let val = s.parse::<T>().map_err(stringify)?;
    check_range(val, min, max)
}

fn check_range<T: Ord + std::fmt::Display>(val: T, min: T, max: T) -> Result<T, String> where T: FromStr, <T as FromStr>::Err: std::fmt::Display {
    if val > max {
        Err(format!("exceeds maximum of {max}"))
    } else if val < min {
        Err(format!("less than minimum of {min}"))
    } else {
        Ok(val)
    }
}

fn stringify<T: std::fmt::Display>(e: T) -> String {
    format!("{e}")
}
//

pub fn parse_args() -> Args {
    let mut args = Args::parse();
    if args.undo {
        args.no_buf = true;
    }
    return args;
}
