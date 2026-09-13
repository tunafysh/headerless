use anyhow::Result;
use clap::Parser;
use globwalk::GlobWalkerBuilder;

use crate::args::Args;
use crate::generator::Generator;
use crate::log::init_logging;

mod args;
mod generator;
mod log;
mod parsers;

fn main() -> Result<()> {
    init_logging();
    let args = Args::parse();
    let files = GlobWalkerBuilder::from_patterns(&args.source, &["**/*.c"])
        .build()?
        .filter_map(Result::ok)
        .map(|entry| entry.into_path())
        .collect::<Vec<_>>();
    let mut generator = Generator::new(files, args.source, args.output);

    generator.generate()?;
    Ok(())
}
