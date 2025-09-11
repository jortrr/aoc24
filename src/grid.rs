use std::fmt::{self, Debug};
use std::str::FromStr;

use crate::solver::Int;

pub type Point = (Int, Int);

pub struct Grid<T: FromStr> {
    cols: usize,
    data: Vec<Vec<T>>,
    rows: usize,
}

impl<T: FromStr> Grid<T> {
    pub fn cols(&self) -> usize {
        self.rows
    }

    pub fn get<X, Y>(&self, x: X, y: Y) -> Option<&T>
    where
        X: TryInto<usize>,
        Y: TryInto<usize>,
    {
        let x: usize = x.try_into().ok()?;
        let y: usize = y.try_into().ok()?;

        if x >= self.cols || y >= self.rows {
            return None;
        }

        Some(self.data.get(y)?.get(x)?)
    }

    pub fn get_by_point<X, Y>(
        &self,
        p: (X, Y),
    ) -> Option<&T>
    where
        X: TryInto<usize>,
        Y: TryInto<usize>,
    {
        self.get(p.0, p.1)
    }

    pub fn parse(
        data: Vec<String>,
    ) -> Result<Self, T::Err> {
        let data: Vec<Vec<T>> = data
            .iter()
            .map(|line| {
                line.chars()
                    .map(|ch| {
                        T::from_str(&ch.to_string())
                    })
                    .collect()
            })
            .collect::<Result<_, _>>()?;
        let cols: usize = data.get(0).unwrap().len();
        let rows: usize = data.len();
        assert!(
            data.iter().all(|line| line.len() == cols)
        );
        Ok(Self { cols, data, rows })
    }

    pub fn rows(&self) -> usize {
        self.rows
    }
}

impl Debug for Grid<char> {
    fn fmt(
        &self,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        f.debug_struct("Grid")
            .field("rows", &self.rows)
            .field("cols", &self.cols)
            .field(
                "data",
                &self
                    .data
                    .iter()
                    .map(|row| {
                        row.iter().collect::<String>()
                    })
                    .collect::<Vec<_>>(),
            )
            .finish()
    }
}
