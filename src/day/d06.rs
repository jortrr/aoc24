use std::{collections::HashSet, error::Error};

use nalgebra::Vector2;

use crate::{
    file::read_file_by_non_empty_line,
    grid::{Grid, GridPoint},
    solver::{self, Int, Solution},
};

pub struct Solver;

fn rotate(vector: Vector2<Int>) -> Vector2<Int> {
    Vector2::new(-vector.y, vector.x)
}

impl solver::Solver for Solver {
    fn run() -> Result<Solution, Box<dyn Error>> {
        let grid: Grid<char> = Grid::parse(
            read_file_by_non_empty_line("06.txt")?,
        )?;
        let start: GridPoint<char> = grid
            .points()
            .into_iter()
            .find(|p| p.value == '^')
            .ok_or("Cannot find start point")?;
        let mut step = Vector2::new(0, -1);
        let mut current = start.point;
        let mut visited: HashSet<Vector2<Int>> =
            HashSet::new();
        while let Some(value) =
            grid.at(&(current + step))
        {
            visited.insert(current);
            if *value == '#' {
                step = rotate(step);
            }
            current = current + step;
        }
        visited.insert(current);
        dbg!(visited.len());
        Err("".into())
    }
}
