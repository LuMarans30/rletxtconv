use lazy_static::lazy_static;
use regex::Regex;
use std::io::Write;

use crate::{ConwayError, Result, filter_comment_lines, universe::Universe};

lazy_static! {
    pub static ref RLE_HEADER_REGEX: Regex =
        Regex::new(r"^\s*x\s*=\s*(\d+)\s*,\s*y\s*=\s*(\d+)").unwrap();
}

/// Check if a line is a valid RLE header
pub fn is_valid_header(line: &str) -> bool {
    RLE_HEADER_REGEX.is_match(line)
}

/// Parse content in RLE format
pub fn parse(content: &str) -> Result<Universe> {
    let lines = filter_comment_lines(content, '#');

    if lines.is_empty() {
        return Err(ConwayError::Parsing("No valid content found".into()));
    }

    // Parse header
    let (width, height) = extract_dimensions(lines[0]).map_err(ConwayError::Parsing)?;

    let mut universe = Universe::new(width, height);

    // Parse data lines (skip the header)
    let data = lines[1..].join("");
    let data = data.split('!').next().unwrap_or("");

    let mut curr_row = Vec::with_capacity(width);
    let mut count_str = String::new();

    for c in data.chars() {
        match c {
            '$' => {
                let count = if count_str.is_empty() {
                    1
                } else {
                    count_str.parse().map_err(|e| {
                        ConwayError::Parsing(format!("Invalid count before $: {e}"))
                    })?
                };
                count_str.clear();

                for _ in 0..count {
                    curr_row.resize(width, false);
                    universe.cells.extend(&curr_row);
                    curr_row.clear();
                }
            }
            c @ ('o' | 'O' | 'b' | 'B') => {
                let alive = matches!(c, 'o' | 'O');
                let count = count_str.parse().unwrap_or(1);
                count_str.clear();
                curr_row.extend(std::iter::repeat_n(alive, count));
            }
            c if c.is_ascii_digit() => {
                count_str.push(c);
            }
            _ => continue, // Ignore other characters
        }
    }

    // Handle any remaining cells in the last row
    if !curr_row.is_empty() {
        curr_row.resize(width, false);
        universe.cells.extend(&curr_row);
    }

    // Pad with empty rows if needed
    let expected_size = width * height;
    if universe.cells.len() < expected_size {
        universe.cells.resize(expected_size, false);
    }

    Ok(universe)
}

/// Write universe in RLE format
pub fn write(universe: &Universe, writer: &mut dyn Write) -> Result<()> {
    // Write header
    writeln!(
        writer,
        "x = {}, y = {}, rule = B3/S23",
        universe.width, universe.height
    )?;

    let mut line_length = 0;
    let mut curr_run = None;
    let mut run_count = 0;

    for row in 0..universe.height {
        for col in 0..universe.width {
            let idx = row * universe.width + col;
            let cell = *universe.cells.get(idx).unwrap_or(&false);

            if curr_run == Some(cell) {
                run_count += 1;
            } else {
                if let Some(run_val) = curr_run.take() {
                    write_run(writer, run_val, run_count, &mut line_length)?;
                }
                curr_run = Some(cell);
                run_count = 1;
            }
        }

        // Write any pending run at the end of a row
        if let Some(run_val) = curr_run {
            write_run(writer, run_val, run_count, &mut line_length)?;
            curr_run = None;
            run_count = 0;
        }

        write_symbol(
            writer,
            if row < universe.height - 1 { '$' } else { '!' }, // End of row marker or pattern
            &mut line_length,
        )?;
    }

    Ok(())
}

/// Write a run of cells
fn write_run(
    writer: &mut dyn Write,
    cell: bool,
    count: usize,
    line_length: &mut usize,
) -> Result<()> {
    let mut output = String::new();

    if count > 1 {
        output.push_str(&count.to_string());
    }

    output.push(if cell { 'o' } else { 'b' });

    // Wrap line if it would exceed 70 characters
    if *line_length + output.len() > 70 {
        writeln!(writer)?;
        *line_length = 0;
    }

    write!(writer, "{output}")?;
    *line_length += output.len();

    Ok(())
}

/// Write a symbol ($ or !) with line wrapping
fn write_symbol(writer: &mut dyn Write, symbol: char, line_length: &mut usize) -> Result<()> {
    if *line_length + 1 > 70 {
        writeln!(writer)?;
        *line_length = 0;
    }

    write!(writer, "{symbol}")?;
    *line_length += 1;

    Ok(())
}

/// Extract width and height from an RLE header
fn extract_dimensions(header: &str) -> std::result::Result<(usize, usize), String> {
    let captures = RLE_HEADER_REGEX
        .captures(header)
        .ok_or_else(|| "Couldn't extract dimensions".to_string())?;

    let width = captures
        .get(1)
        .and_then(|m| m.as_str().parse::<usize>().ok())
        .ok_or_else(|| "Invalid width in header".to_string())?;

    let height = captures
        .get(2)
        .and_then(|m| m.as_str().parse::<usize>().ok())
        .ok_or_else(|| "Invalid height in header".to_string())?;

    if width == 0 || height == 0 {
        return Err("Dimensions cannot be zero".to_string());
    }

    Ok((width, height))
}
