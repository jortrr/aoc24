use crate::{
    file::read_file_by_non_empty_line,
    grid::Grid,
    solver::{self, Int, Solution},
};
use std::error::Error;

pub struct Solver;

type Point = (Int, Int);

fn spells_mas(
    a: Point,
    b: Point,
    c: Point,
    grid: &Grid<char>,
) -> bool {
    match (
        grid.get_by_point(a),
        grid.get_by_point(b),
        grid.get_by_point(c),
    ) {
        (Some(&a), Some(&b), Some(&c)) => {
            (a, b, c) == ('M', 'A', 'S')
        }
        (_, _, _) => false,
    }
}

fn direction(
    x: Int,
    y: Int,
    dx: Int,
    dy: Int,
) -> [Point; 3] {
    [
        (x + dx, y + dy),
        (x + 2 * dx, y + 2 * dy),
        (x + 3 * dx, y + 3 * dy),
    ]
}

impl solver::Solver for Solver {
    fn run() -> Result<Solution, Box<dyn Error>> {
        let file: Vec<String> =
            read_file_by_non_empty_line("04.txt")?;
        dbg!(&file);
        let grid: Grid<char> = Grid::parse(file)?;
        dbg!(&grid);
        let mut xmas_counter = 0;

        for x in 0..grid.cols() as Int {
            for y in 0..grid.rows() as Int {
                let value: char =
                    *grid.get(x, y).unwrap();
                if value != 'X' {
                    continue;
                }
                let directions: [[Point; 3]; 8] = [
                    direction(x, y, 1, 0),
                    direction(x, y, 1, -1),
                    direction(x, y, 0, -1),
                    direction(x, y, -1, -1),
                    direction(x, y, -1, 0),
                    direction(x, y, -1, 1),
                    direction(x, y, 0, 1),
                    direction(x, y, 1, 1),
                ];
                for p in directions {
                    if spells_mas(
                        p[0], p[1], p[2], &grid,
                    ) {
                        xmas_counter += 1;
                    }
                }
            }
        }
        dbg!(xmas_counter);

        Err("TODO".into())
    }
}
