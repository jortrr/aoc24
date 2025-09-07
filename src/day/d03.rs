use std::error::Error;

use regex::Regex;

use crate::{
    file::read_file_by_non_empty_line,
    solver::{self, Int, Solution},
};

pub struct Solver;

impl solver::Solver for Solver {
    fn run() -> Result<Solution, Box<dyn Error>> {
        let file: Vec<String> =
            read_file_by_non_empty_line("03.txt")?;
        let re = Regex::new(r"mul\((\d+),(\d+)\)")?;
        dbg!(&file);
        let mut sum: Int = 0;
        for line in &file {
            for mat in re.captures_iter(line) {
                dbg!(&mat);
                sum += &mat[1].parse::<Int>()?
                    * &mat[2].parse::<Int>()?;
            }
        }
        dbg!(sum);
        Err("TODO".into())
    }
}
