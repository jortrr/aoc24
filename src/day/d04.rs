use crate::{
    file::read_file_by_non_empty_line,
    grid::{Grid, Point},
    solver::{self, Int, Solution},
};
use std::error::Error;

pub struct Solver;

fn spells_mas(
    a: Point,
    b: Point,
    c: Point,
    grid: &Grid<char>,
) -> bool {
    matches!(
        (
            grid.get_by_point(a),
            grid.get_by_point(b),
            grid.get_by_point(c)
        ),
        (Some(&'M'), Some(&'A'), Some(&'S'))
    )
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

fn m_and_s(
    a: Point,
    b: Point,
    grid: &Grid<char>,
) -> bool {
    if let (Some(a), Some(b)) =
        (grid.get_by_point(a), grid.get_by_point(b))
    {
        let letters: String = format!("{}{}", a, b);
        return letters.contains('M')
            && letters.contains('S');
    }
    false
}

impl solver::Solver for Solver {
    fn run() -> Result<Solution, Box<dyn Error>> {
        let file: Vec<String> =
            read_file_by_non_empty_line("04.txt")?;
        dbg!(&file);
        let grid: Grid<char> = Grid::parse(file)?;
        dbg!(&grid);
        let mut xmas_counter: Int = 0;
        let mut cross_xmas_counter: Int = 0;
        for x in 0..grid.cols() as Int {
            for y in 0..grid.rows() as Int {
                let value: char =
                    *grid.get(x, y).unwrap();
                if value == 'X' {
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
                } else if value == 'A' {
                    if m_and_s(
                        (x - 1, y - 1),
                        (x + 1, y + 1),
                        &grid,
                    ) && m_and_s(
                        (x - 1, y + 1),
                        (x + 1, y - 1),
                        &grid,
                    ) {
                        cross_xmas_counter += 1;
                    }
                }
            }
        }
        Ok(Solution {
            part_one: xmas_counter,
            part_two: cross_xmas_counter,
        })
    }
}
