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
                let value: char = *grid.get(x, y).unwrap();
                if value != 'X' {
                    continue;
                }
                let directions: [[Point; 3]; 8] = [
                    [(x + 1, y), (x + 2, y), (x + 3, y)],
                    [
                        (x + 1, y - 1),
                        (x + 2, y - 2),
                        (x + 3, y - 3),
                    ],
                    [(x, y - 1), (x, y - 2), (x, y - 3)],
                    [
                        (x - 1, y - 1),
                        (x - 2, y - 2),
                        (x - 3, y - 3),
                    ],
                    [(x - 1, y), (x - 2, y), (x - 3, y)],
                    [
                        (x - 1, y + 1),
                        (x - 2, y + 2),
                        (x - 3, y + 3),
                    ],
                    [(x, y + 1), (x, y + 2), (x, y + 3)],
                    [
                        (x + 1, y + 1),
                        (x + 2, y + 2),
                        (x + 3, y + 3),
                    ],
                ];
                for p in directions {
                    if spells_mas(p[0], p[1], p[2], &grid) {
                        xmas_counter += 1;
                    }
                }
            }
        }
        dbg!(xmas_counter);

        Err("TODO".into())
    }
}
