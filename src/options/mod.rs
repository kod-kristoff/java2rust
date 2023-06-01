use std::path::PathBuf;

use clap::Parser;

/// Doc comments will appear in your application's help text.
/// Multi-line comments are also supported.
#[derive(Debug, Parser)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    /// Optional output path.
    #[clap(long, short)]
    pub output: Option<std::path::PathBuf>,

    /// source path.
    pub source: std::path::PathBuf,

    /// Enable verbose output.
    #[clap(long, short, action = clap::ArgAction::Set, default_value_t = false)]
    pub verbose: bool,
}
