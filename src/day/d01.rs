use crate::{
    file::read_file_by_non_empty_line,
    parse::parse_each_word_as,
    solver::{self, Int, Solution},
};
use std::error::Error;

pub struct Solver;

impl solver::Solver for Solver {
    fn run() -> Result<Solution, Box<dyn Error>> {
        println!("Day 1");
        let file: Vec<String> =
            read_file_by_non_empty_line("01.txt")?;
        dbg!(&file);
        let mut left_list: Vec<Int> = Vec::new();
        let mut right_list: Vec<Int> = Vec::new();
        for line in &file {
            let numbers: Vec<Int> =
                parse_each_word_as::<Int>(&line)?;
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
                .count()
                as Int;
            similarity += left * right_count;
        }
        dbg!(&similarity);
        Ok(Solution {
            part_one: distance,
            part_two: similarity,
        })
    }
}
