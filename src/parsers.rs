use anyhow::Result;
use nom::{
    Parser,
    branch::alt,
    bytes::complete::take_until,
    character::complete::{anychar, char},
    combinator::value,
    multi::many0,
};

fn has_semicolon(input: &str) -> bool {
    take_until::<_, _, nom::error::Error<&str>>(";")
        .parse(input)
        .is_ok_and(|(_, rest)| rest.starts_with(';'))
}

fn brace_delta(input: &str) -> i32 {
    many0(alt((
        value(1, char::<_, nom::error::Error<&str>>('{')),
        value(-1, char::<_, nom::error::Error<&str>>('}')),
        value(0, anychar::<_, nom::error::Error<&str>>),
    )))
    .parse(input)
    .map(|(_, values)| values.into_iter().sum())
    .unwrap_or(0)
}

fn paren_delta(input: &str) -> i32 {
    many0(alt((
        value(1, char::<_, nom::error::Error<&str>>('(')),
        value(-1, char::<_, nom::error::Error<&str>>(')')),
        value(0, anychar::<_, nom::error::Error<&str>>),
    )))
    .parse(input)
    .map(|(_, values)| values.into_iter().sum())
    .unwrap_or(0)
}

fn is_aggregate(input: &str) -> bool {
    alt((
        take_until::<_, _, nom::error::Error<&str>>("struct"),
        take_until::<_, _, nom::error::Error<&str>>("union"),
        take_until::<_, _, nom::error::Error<&str>>("enum"),
    ))
    .parse(input)
    .is_ok()
}

fn aggregate_parser<'a>(
    first_line: &'a str,
    lines: &mut impl Iterator<Item = (usize, &'a str)>,
) -> Result<String> {
    let mut definition = String::from(first_line);
    definition.push('\n');

    let mut braces = brace_delta(first_line);

    while let Some((_, line)) = lines.next() {
        definition.push_str(line);
        definition.push('\n');
        braces += brace_delta(line);

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

    if has_semicolon(first_line) {
        return Ok(definition);
    }

    while let Some((_, line)) = lines.next() {
        definition.push_str(line);
        definition.push('\n');

        if has_semicolon(line) {
            break;
        }
    }

    Ok(definition)
}

pub fn typedef_parser<'a>(
    first_line: &'a str,
    lines: &mut impl Iterator<Item = (usize, &'a str)>,
) -> Result<String> {
    if is_aggregate(first_line) {
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
    if first_line.trim_end().ends_with(';') {
        let mut declaration = String::from(first_line.trim_end());
        if !declaration.ends_with(';') {
            declaration.push(';');
        }
        declaration.push('\n');
        return Ok(declaration);
    }

    let mut definition = String::from(first_line);
    definition.push('\n');

    let mut depth = paren_delta(first_line);

    if depth == 0 && first_line.contains(')') {
        if let Some(body_start) = definition.find('{') {
            definition.truncate(body_start);
        } else {
            definition.pop();
        }
        definition.push_str(";\n");
        return Ok(definition);
    }

    while let Some((_, line)) = lines.next() {
        definition.push_str(line);
        definition.push('\n');

        depth += paren_delta(line);

        if depth == 0 && line.contains(')') {
            break;
        }
    }

    if let Some(body_start) = definition.find('{') {
        definition.truncate(body_start);
    } else {
        definition.pop();
    }
    definition.push_str(";\n");

    Ok(definition)
}
