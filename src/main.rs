mod day;
mod file;
mod grid;
mod parse;
mod solver;

use crate::day::*;
use crate::solver::Solver;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let d01 = d01::Solver::run()?;
    dbg!(d01);
    let d02 = d02::Solver::run()?;
    dbg!(d02);
    let d03 = d03::Solver::run()?;
    dbg!(d03);
    let d04 = d04::Solver::run()?;
    dbg!(d04);
    Ok(())
}
