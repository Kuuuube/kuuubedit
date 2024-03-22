use clap::Parser;

/// Powerful headless text editor for processing enormous files
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Enables the undo command and --no-buf
    #[arg(short, long)]
    pub undo: bool,

    /// Sets file buffer size
    #[arg(short, long, default_value_t = 100000000)]
    pub buffer: u64,

    /// Disables file buffering
    #[arg(long)]
    pub no_buf: bool,

    /// Path to file to open
    #[arg(value_name = "FILEPATH")]
    pub file: String,
}

pub fn parse_args() -> Args {
    let mut args = Args::parse();
    if args.undo {
        args.no_buf = true;
    }
    return args;
}
