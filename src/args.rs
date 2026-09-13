use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
pub struct Args {
    /// Directory containing C source files
    pub source: PathBuf,

    /// Output directory
    #[arg(short, long, default_value = "build/generated")]
    pub output: PathBuf,
}
