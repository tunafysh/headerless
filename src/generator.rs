use anyhow::{Result, anyhow};
use std::{
    fs::{create_dir_all, read_to_string},
    path::PathBuf,
};
use tracing::{debug, info, instrument};

use crate::parsers;

#[derive(Debug)]
enum PublicKind {
    Typedef,
    Struct,
    Union,
    Enum,
    Extern,
    Function,
}

struct PublicDefinition {
    header: String,
    source: String,
}

struct RecordingLines<'source, I> {
    lines: I,
    source: &'source mut String,
}

impl<'line, 'source, I> Iterator for RecordingLines<'source, I>
where
    I: Iterator<Item = (usize, &'line str)>,
{
    type Item = (usize, &'line str);

    fn next(&mut self) -> Option<Self::Item> {
        let (line_number, line) = self.lines.next()?;
        self.source.push_str(line);
        self.source.push('\n');
        Some((line_number, line))
    }
}

fn classify(line: &str) -> PublicKind {
    match line.split_whitespace().next() {
        Some("typedef") => PublicKind::Typedef,
        Some("struct") => PublicKind::Struct,
        Some("union") => PublicKind::Union,
        Some("enum") => PublicKind::Enum,
        Some("extern") => PublicKind::Extern,
        _ => PublicKind::Function,
    }
}

pub struct Generator {
    files: Vec<PathBuf>,
    source_dir: PathBuf,
    dir: PathBuf,
}

impl Generator {
    #[instrument(
        name = "generator.new",
        skip(files),
        fields(file_count = files.len(), source_dir = %source_dir.display(), output_dir = %dir.display())
    )]
    pub fn new(files: Vec<PathBuf>, source_dir: PathBuf, dir: PathBuf) -> Self {
        Self {
            files,
            source_dir,
            dir,
        }
    }

    #[instrument(
        name = "generator.parse_public",
        skip(self, lines),
        fields(marker_line = marker_line + 1)
    )]
    fn parse_public<'a>(
        &self,
        lines: &mut impl Iterator<Item = (usize, &'a str)>,
        marker_line: usize,
    ) -> Result<PublicDefinition> {
        let (_, first_line) = lines.next().ok_or_else(|| {
            anyhow!(
                "There's no definition under @public at line {}",
                marker_line + 1
            )
        })?;

        let mut source = String::new();
        source.push_str(first_line);
        source.push('\n');

        let mut recording_lines = RecordingLines {
            lines,
            source: &mut source,
        };
        let kind = classify(first_line);
        debug!(?kind, "classified public definition");

        let header = match kind {
            PublicKind::Typedef => parsers::typedef_parser(first_line, &mut recording_lines),
            PublicKind::Struct => parsers::struct_parser(first_line, &mut recording_lines),
            PublicKind::Union => parsers::union_parser(first_line, &mut recording_lines),
            PublicKind::Enum => parsers::enum_parser(first_line, &mut recording_lines),
            PublicKind::Extern => parsers::extern_parser(first_line, &mut recording_lines),
            PublicKind::Function => parsers::function_parser(first_line, &mut recording_lines),
        }?;

        debug!(
            header_bytes = header.len(),
            source_bytes = source.len(),
            "parsed public definition"
        );
        Ok(PublicDefinition { header, source })
    }

    #[instrument(
        name = "generator.generate",
        skip(self),
        fields(
            file_count = self.files.len(),
            source_dir = %self.source_dir.display(),
            output_dir = %self.dir.display()
        )
    )]
    pub fn generate(&mut self) -> Result<()> {
        create_dir_all(&self.dir)?;
        info!("starting source generation");

        for file in &self.files {
            let file_span = tracing::info_span!("generator.file", input = %file.display());
            let _file_span = file_span.enter();
            let relative = file.strip_prefix(&self.source_dir)?;
            let source_path = self.dir.join(relative);
            let header_path = source_path.with_extension("h");
            let file = read_to_string(file)?;
            debug!(source_bytes = file.len(), "read source file");

            let mut lines = file.lines().enumerate();
            let mut generated_source = String::new();
            let mut header_source = String::new();

            while let Some((line_number, line)) = lines.next() {
                let line = line.trim();
                if line == "@public" || line.strip_prefix("//").map(str::trim) == Some("@public") {
                    let definition = self.parse_public(&mut lines, line_number)?;
                    header_source.push_str(format!("{}\n", &definition.header).as_str());
                    generated_source.push_str(&definition.source);
                    continue;
                }

                generated_source.push_str(line);
                generated_source.push('\n');
            }

            if let Some(parent) = source_path.parent() {
                create_dir_all(parent)?;
            }

            std::fs::write(source_path, &generated_source)?;
            std::fs::write(header_path, &header_source)?;
            info!(
                generated_source_bytes = generated_source.len(),
                generated_header_bytes = header_source.len(),
                "wrote generated files"
            );
        }

        info!("finished source generation");
        Ok(())
    }
}
