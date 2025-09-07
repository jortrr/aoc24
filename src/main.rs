mod day;
mod file;
mod parse;
mod solver;

use crate::day::*;
use crate::solver::Solver;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let d01 = d01::Solver::run()?;
    let d02 = d02::Solver::run()?;
    dbg!(d01);
    dbg!(d02);
    Ok(())
}
