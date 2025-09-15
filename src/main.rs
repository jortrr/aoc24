mod day;
mod file;
mod grid;
mod parse;
mod solver;

use crate::day::*;
use crate::solver::{Int, Solver};
use clap::Parser;
use std::error::Error;

#[derive(Parser)]
#[command(name = "aoc24")]
#[command(about = "Advent of Code 2024 - jortrr", long_about = None)]
struct Cli {
    /// Day to run, one of {1, ..., 25}
    #[arg(short, long)]
    day: Int,
}

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();
    println!(
        "{:#?}",
        match cli.day {
            1 => d01::Solver::run()?,
            2 => d02::Solver::run()?,
            3 => d03::Solver::run()?,
            4 => d04::Solver::run()?,
            5 => d05::Solver::run()?,
            _ => panic!(
                "Day not supported: {}",
                cli.day
            ),
        }
    );
    Ok(())
}
