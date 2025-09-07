use std::error::Error;

use crate::file::read_file_by_line;

type Int = i32;

pub fn run() -> Result<(), Box<dyn Error>> {
    println!("Day 2");
    let file: Vec<String> = read_file_by_line("02.txt")?;
    dbg!(&file);
    let mut safe_reports: Int = 0;
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
        let increasing: bool = numbers[1] > numbers[0];
        let mut safe: bool = true;
        for window in numbers.windows(2) {
            let diff = window[1] - window[0];
            if (increasing && (diff < 1 || diff > 3))
                || (!increasing && (diff > -1 || diff < -3))
            {
                println!("Unsafe:\t{}", line);
                safe = false;
                break;
            }
        }
        if safe {
            println!("Safe:\t{}", line);
            safe_reports += 1;
        }
    }
    dbg!(safe_reports);
    Ok(())
}
