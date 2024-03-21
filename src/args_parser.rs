use clap::Parser;

/// Powerful headless text editor for processing enormous files
#[derive(Parser, Debug)]
#[command(about, long_about = None)]
pub struct Args {
    /// Enables the undo command
    #[arg(short, long)]
    pub undo: bool,

    /// Path to file to open
    pub file: String,
}

pub fn parse_args() -> Args {
    return Args::parse();
}