use anyhow::Result;

fn aggregate_parser<'a>(
    first_line: &'a str,
    lines: &mut impl Iterator<Item = (usize, &'a str)>,
) -> Result<String> {
    let mut definition = String::from(first_line);
    definition.push('\n');

    let mut braces = first_line.matches('{').count() - first_line.matches('}').count();

    while let Some((_, line)) = lines.next() {
        definition.push_str(line);
        definition.push('\n');

        braces += line.matches('{').count();
        braces -= line.matches('}').count();

        if braces == 0 {
            break;
        }
    }

    Ok(definition)
}

fn semicolon_parser<'a>(
    first_line: &'a str,
    lines: &mut impl Iterator<Item = (usize, &'a str)>,
) -> Result<String> {
    let mut definition = String::from(first_line);
    definition.push('\n');

    if first_line.trim_end().ends_with(';') {
        return Ok(definition);
    }

    while let Some((_, line)) = lines.next() {
        definition.push_str(line);
        definition.push('\n');

        if line.trim_end().ends_with(';') {
            break;
        }
    }

    Ok(definition)
}

pub fn typedef_parser<'a>(
    first_line: &'a str,
    lines: &mut impl Iterator<Item = (usize, &'a str)>,
) -> Result<String> {
    let is_aggregate = first_line
        .split_whitespace()
        .any(|word| matches!(word, "struct" | "union" | "enum"));
    if is_aggregate {
        let mut definition = aggregate_parser(first_line, lines)?;

        if !definition.trim_end().ends_with(';') {
            while let Some((_, line)) = lines.next() {
                definition.push_str(line);
                definition.push('\n');

                if line.trim_end().ends_with(';') {
                    break;
                }
            }
        }

        return Ok(definition);
    }

    semicolon_parser(first_line, lines)
}

pub fn struct_parser<'a>(
    first_line: &'a str,
    lines: &mut impl Iterator<Item = (usize, &'a str)>,
) -> Result<String> {
    aggregate_parser(first_line, lines)
}

pub fn union_parser<'a>(
    first_line: &'a str,
    lines: &mut impl Iterator<Item = (usize, &'a str)>,
) -> Result<String> {
    aggregate_parser(first_line, lines)
}

pub fn enum_parser<'a>(
    first_line: &'a str,
    lines: &mut impl Iterator<Item = (usize, &'a str)>,
) -> Result<String> {
    aggregate_parser(first_line, lines)
}

pub fn extern_parser<'a>(
    first_line: &'a str,
    lines: &mut impl Iterator<Item = (usize, &'a str)>,
) -> Result<String> {
    semicolon_parser(first_line, lines)
}

pub fn function_parser<'a>(
    first_line: &'a str,
    lines: &mut impl Iterator<Item = (usize, &'a str)>,
) -> Result<String> {
    let mut definition = String::from(first_line);
    definition.push('\n');

    let mut depth = first_line.matches('(').count() as i32 - first_line.matches(')').count() as i32;

    if depth == 0 && first_line.contains(')') {
        definition.pop();
        definition.push(';');
        definition.push('\n');
        return Ok(definition);
    }

    while let Some((_, line)) = lines.next() {
        definition.push_str(line);
        definition.push('\n');

        depth += line.matches('(').count() as i32;
        depth -= line.matches(')').count() as i32;

        if depth == 0 && line.contains(')') {
            break;
        }
    }

    definition.pop();
    definition.push(';');
    definition.push('\n');

    Ok(definition)
}
