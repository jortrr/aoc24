use crate::{
    file::read_file_by_non_empty_line,
    solver::{self, Int, Solution},
};
use regex::Regex;
use std::error::Error;

pub struct Solver;

impl solver::Solver for Solver {
    fn run() -> Result<Solution, Box<dyn Error>> {
        let file: Vec<String> =
            read_file_by_non_empty_line("03.txt")?;
        let re = Regex::new(
            r"mul\((\d+),(\d+)\)|do\(\)|don't\(\)",
        )?;
        dbg!(&file);
        let mut sum: Int = 0;
        let mut enabled_sum: Int = 0;
        let mut enabled: bool = true;
        for line in &file {
            for mat in re.captures_iter(line) {
                dbg!(&mat);
                if &mat[0] == "do()" {
                    enabled = true;
                } else if &mat[0] == "don't()" {
                    enabled = false;
                } else {
                    let multiplication = &mat[1]
                        .parse::<Int>()?
                        * &mat[2].parse::<Int>()?;
                    sum += multiplication;
                    if enabled {
                        enabled_sum += multiplication;
                    }
                }
            }
        }
        dbg!(&sum);
        dbg!(&enabled_sum);
        Ok(Solution {
            part_one: sum,
            part_two: enabled_sum,
        })
    }
}
