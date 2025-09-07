use std::error::Error;

pub type Int = i32;

#[derive(Debug)]
pub struct Solution {
    pub part_one: Int,
    pub part_two: Int,
}

pub trait Solver {
    fn run() -> Result<Solution, Box<dyn Error>>;
}
