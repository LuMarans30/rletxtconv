use std::io::Write;

use crate::{ConwayError, Result, filter_comment_lines, universe::Universe};

/// Parse content in plaintext format
pub fn parse(content: &str) -> Result<Universe> {
    let lines = filter_comment_lines(content, '!');

    if lines.is_empty() {
        return Err(ConwayError::Parsing("No valid content found".into()));
    }

    // Determine dimensions
    let height = lines.len();
    let width = lines
        .iter()
        .map(|line| line.chars().filter(|&c| c == 'O' || c == '.').count())
        .max()
        .unwrap_or(0);

    if width == 0 {
        return Err(ConwayError::Parsing("No valid cells found".into()));
    }

    let mut universe = Universe::new(width, height);

    for line in lines {
        let mut row = Vec::with_capacity(width);

        // Process each character in the line
        for c in line.chars() {
            match c {
                'O' => row.push(true),
                '.' => row.push(false),
                _ => continue, // Skip invalid characters
            }
        }

        // Pad row to full width if needed
        row.resize(width, false);
        universe.cells.extend(row);
    }

    Ok(universe)
}

/// Write universe in plaintext format
pub fn write(universe: &Universe, writer: &mut dyn Write) -> Result<()> {
    for row in 0..universe.height {
        for col in 0..universe.width {
            let idx = row * universe.width + col;
            let cell = universe.cells.get(idx).copied().unwrap_or(false);

            write!(writer, "{}", if cell { 'O' } else { '.' })?;
        }
        writeln!(writer)?;
    }

    Ok(())
}
