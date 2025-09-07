use std::error::Error;

use crate::file::read_file_by_line;

type Int = i32;

pub fn safe(numbers: &Vec<Int>) -> bool {
    let increasing: bool = numbers[1] > numbers[0];
    let mut safe: bool = true;
    for window in numbers.windows(2) {
        let diff = window[1] - window[0];
        if (increasing && (diff < 1 || diff > 3))
            || (!increasing && (diff > -1 || diff < -3))
        {
            safe = false;
            break;
        }
    }
    safe
}

pub fn run() -> Result<(), Box<dyn Error>> {
    println!("Day 2");
    let file: Vec<String> = read_file_by_line("02.txt")?;
    dbg!(&file);
    let mut safe_reports: Int = 0;
    let mut damp_safe_reports: Int = 0;
    for line in &file {
        if line.is_empty() {
            continue;
        }
        let numbers: Vec<Int> = line
            .split_whitespace()
            .map(|number| number.parse::<Int>().unwrap())
            .collect();
        if numbers.len() < 2 {
            return Err(format!(
                "Expected at least 2 numbers but found \
                '{}' in '{}'",
                numbers.len(),
                line
            )
            .into());
        }
        let is_safe: bool = safe(&numbers);
        let mut is_damp_safe: bool = false;
        if !is_safe {
            for i in 0..numbers.len() {
                let mut numbers_clone = numbers.clone();
                numbers_clone.remove(i);
                if safe(&numbers_clone) {
                    is_damp_safe = true;
                    break;
                }
            }
        }
        if is_safe {
            println!("Safe:\t\t{}", line);
            safe_reports += 1;
            damp_safe_reports += 1;
        } else if is_damp_safe {
            println!("Damp safe:\t{}", line);
            damp_safe_reports += 1;
        } else {
            println!("Unsafe:\t\t{}", line);
        }
    }
    dbg!(safe_reports);
    dbg!(damp_safe_reports);
    Ok(())
}
