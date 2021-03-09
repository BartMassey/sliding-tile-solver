use ndarray::prelude::*;

use crate::*;

#[derive(Debug)]
pub struct ParseError;

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "parse error")
    }
}

impl std::error::Error for ParseError {}

pub fn load_puzzle<P>(path: P) -> Result<Puzzle, Box<dyn std::error::Error>>
    where P: AsRef<std::path::Path>
{
    let text = std::fs::read_to_string(path)?;
    let lines: Vec<Vec<char>> = text
        .split('\n')
        .map(|l| l.trim())
        .filter(|l| l.len() > 0)
        .map(|s| s.chars().collect())
        .collect();
    let n = lines.len();
    let max_width = lines.iter().map(|s| s.len()).max().unwrap();
    let (num_width, field_width, target_width) = if n <= 3 {
        (1, 1, n)
    } else {
        (2, 3, 3 * n - 1)
    };
    if max_width != target_width {
        return Err(ParseError.into());
    }
    let mut digits: Vec<u8> = Vec::with_capacity(n * n);
    for r in 0..n {
        let line = &lines[r];
        let nline = line.len();
        for c in 0..n {
            let c_index = c * field_width;
            if c_index + num_width > nline {
                if c == n - 1 {
                    digits.push(0);
                    continue;
                }
                return Err(ParseError.into());
            }
            if line[c_index+num_width-1] == ' ' {
                digits.push(0);
                continue;
            }
            let field: String = line[c_index..c_index+num_width].iter().collect();
            digits.push(field.parse()?);
        }
    }
    if digits.len() != n * n {
        return Err(ParseError.into());
    }
    let mut checkage = digits.clone();
    checkage.sort();
    for (i, c) in checkage.into_iter().enumerate() {
        if i != c as usize {
            return Err(ParseError.into());
        }
    }
    let tiles = Array::from(digits).into_shape((n, n))?;
    let mut blank_pos = None;
    for r in 0..n {
        for c in 0..n {
            if tiles[(r, c)] == 0 {
                blank_pos = Some((r, c));
            }
        }
    }
    let blank_pos = blank_pos.unwrap();
    Ok(Puzzle { n, tiles, blank_pos })
}
