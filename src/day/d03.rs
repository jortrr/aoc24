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
        let mul_re = Regex::new(r"mul\((\d+),(\d+)\)")?;
        let do_re = Regex::new(r"do\(\)")?;
        let dont_re = Regex::new(r"don't\(\)")?;
        dbg!(&file);
        let mut sum: Int = 0;
        let mut enabled_sum: Int = 0;
        for line in &file {
            let mut do_id: Vec<usize> = Vec::new();
            let mut dont_id: Vec<usize> = Vec::new();
            for mat in do_re.find_iter(line) {
                dbg!(&mat);
                do_id.push(mat.start());
            }
            for mat in dont_re.find_iter(line) {
                dbg!(&mat);
                dont_id.push(mat.start());
            }
            dbg!(&do_id);
            dbg!(&dont_id);
            for mat in mul_re.captures_iter(line) {
                dbg!(&mat);
                let start = mat.get(0).unwrap().start();
                dbg!(start);
                let highest_do = do_id
                    .iter()
                    .filter(|id| **id < start)
                    .max();
                if let Some(highest_do) = highest_do {
                    dbg!(highest_do);
                }
                let highest_dont = dont_id
                    .iter()
                    .filter(|id| **id < start)
                    .max();
                if let Some(highest_dont) = highest_dont {
                    dbg!(highest_dont);
                }
                let multiplication = &mat[1]
                    .parse::<Int>()?
                    * &mat[2].parse::<Int>()?;
                sum += multiplication;
                match (highest_do, highest_dont) {
                    (Some(do_id), Some(dont_id)) => {
                        if dont_id > do_id {
                            continue;
                        }
                    }
                    (None, Some(_)) => {
                        continue;
                    }
                    (_, _) => {}
                }
                println!("Enabled: {}", multiplication);
                enabled_sum += multiplication;
            }
        }
        dbg!(sum);
        dbg!(enabled_sum);
        Err("TODO".into())
    }
}
