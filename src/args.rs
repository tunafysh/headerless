use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
pub struct Args {
    /// Source files or directories
    pub source: Vec<PathBuf>,

    /// Output directory
    #[arg(short, long, default_value = "build/generated")]
    pub output: PathBuf,
}
