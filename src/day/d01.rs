use std::error::Error;
use std::fs::{self};
use std::path::{Path, PathBuf};

type Int = i32;

/// Read file located in resources, split the file by line,
/// and return the resulting vector of lines.
fn read_file_by_line(
    file_name: &str,
) -> Result<Vec<String>, Box<dyn Error>> {
    let path: PathBuf =
        fs::canonicalize(Path::new("resources/"))?
            .join(file_name);
    let file: Vec<String> = fs::read_to_string(path)?
        .split('\n')
        .map(str::to_string)
        .collect();
    Ok(file)
}

pub fn run() -> Result<(), Box<dyn Error>> {
    println!("Day 1");
    let file: Vec<String> = read_file_by_line("01.txt")?;
    dbg!(&file);
    let mut left_list: Vec<Int> = Vec::new();
    let mut right_list: Vec<Int> = Vec::new();
    for line in &file {
        if line.is_empty() {
            continue;
        }
        let numbers: Vec<Int> = line
            .split_whitespace()
            .map(|number| number.parse::<Int>().unwrap())
            .collect();
        if numbers.len() != 2 {
            return Err(format!(
                "Expected 2 numbers in '{}', but found '{}'",
                line,
                numbers.len()
            ).into());
        }
        left_list.push(numbers[0]);
        right_list.push(numbers[1]);
    }
    left_list.sort();
    right_list.sort();
    let mut distance: Int = 0;
    for (left, right) in
        left_list.iter().zip(right_list.iter())
    {
        dbg!((left, right));
        distance += (left - right).abs();
    }
    dbg!(&distance);
    let mut similarity: Int = 0;
    for left in left_list {
        let right_count: Int = right_list
            .iter()
            .filter(|right| **right == left)
            .count() as Int;
        similarity += left * right_count;
    }
    dbg!(&similarity);
    Ok(())
}
