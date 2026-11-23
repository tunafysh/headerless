use anyhow::{Context, Result, anyhow};
use clap::Parser;
use globwalk::GlobWalkerBuilder;
use std::path::{Path, PathBuf};

use crate::args::Args;
use crate::generator::Generator;
use crate::log::init_logging;

mod args;
mod generator;
mod log;
mod parsers;

fn common_file_root(sources: &[PathBuf]) -> Option<PathBuf> {
    let mut root = sources
        .iter()
        .filter(|source| source.is_file())
        .filter_map(|source| source.parent())
        .next()?
        .to_path_buf();

    for source in sources.iter().filter(|source| source.is_file()) {
        while !source.starts_with(&root) {
            if !root.pop() {
                return None;
            }
        }
    }

    Some(root)
}

fn collect_files(sources: &[PathBuf]) -> Result<Vec<(PathBuf, PathBuf)>> {
    let mut files = Vec::new();
    let file_root = common_file_root(sources);

    for source in sources {
        if source.is_file() {
            let relative = file_root
                .as_deref()
                .and_then(|root| source.strip_prefix(root).ok())
                .map(Path::to_path_buf)
                .or_else(|| source.file_name().map(PathBuf::from))
                .ok_or_else(|| anyhow!("Source path has no file name: {}", source.display()))?;
            files.push((source.clone(), relative));
            continue;
        }

        if !source.is_dir() {
            return Err(anyhow!("Source path does not exist: {}", source.display()));
        }

        let source = source.canonicalize()?;

        let entries = GlobWalkerBuilder::from_patterns(&source, &["**/*.c"])
            .build()
            .with_context(|| format!("Failed to scan source directory {}", &source.display()))?;

        for entry in entries {
            let path = entry
                .with_context(|| format!("Failed to read source directory {}", &source.display()))?
                .into_path();
            let relative_path = path
                .strip_prefix(&source)
                .with_context(|| {
                    format!(
                        "Could not determine path of {} relative to {}",
                        path.display(),
                        source.display()
                    )
                })?
                .to_path_buf();
            files.push((path, relative_path));
        }
    }

    Ok(files)
}

fn main() -> Result<()> {
    init_logging();
    let args = Args::parse();
    let files = collect_files(&args.source)?;
    let mut generator = Generator::new(files, args.output);

    generator.generate()?;
    Ok(())
}
