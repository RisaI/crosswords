mod size;
mod solvers;
mod utils;

pub use size::EstimateSize;
pub use solvers::*;

use std::io::BufRead;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Direction {
    Right,
    Down,
    Diagonal,
    AntiDiagonal,
}

impl Direction {
    pub const ALL: [Direction; 4] = [Self::Right, Self::Down, Self::Diagonal, Self::AntiDiagonal];

    pub fn shift_point(self, point: (usize, usize), len: usize) -> (usize, usize) {
        match self {
            Self::Right => (point.0, point.1 + len),
            Self::Down => (point.0 + len, point.1),
            Self::Diagonal => (point.0 + len, point.1 + len),
            Self::AntiDiagonal => (point.0 + len, point.1 - len),
        }
    }

    pub fn shift_point_bounded(
        self,
        point: (usize, usize),
        len: usize,
        bounds: (usize, usize),
    ) -> Option<(usize, usize)> {
        if self == Direction::AntiDiagonal && point.1 < len {
            return None;
        }

        let (row, col) = self.shift_point(point, len);

        if row >= bounds.0 || col >= bounds.1 {
            return None;
        }

        Some((row, col))
    }
}

#[derive(Clone, PartialEq, Eq)]
pub struct Crossword {
    pub rows: usize,
    pub data: Box<[u8]>,
}

impl EstimateSize for Crossword {
    fn estimate_size(&self) -> usize {
        self.rows.estimate_size() + self.data.estimate_size()
    }
}

impl Crossword {
    pub fn new(rows: usize, data: Box<[u8]>) -> Self {
        assert_eq!(
            data.len() % rows,
            0,
            "data length must be a multiple of rows"
        );

        Self { rows, data }
    }

    pub fn parse(reader: impl BufRead) -> anyhow::Result<Self> {
        let mut data = vec![];

        let mut cols = 0;

        for row in reader.lines() {
            let row = row?;

            if row.is_empty() {
                continue;
            }

            if cols == 0 {
                cols = row.len();
            }

            if cols != row.len() {
                anyhow::bail!("inconsistent row length");
            }

            data.extend(row.as_bytes().iter().copied());
        }

        Ok(Self::new(data.len() / cols, data.into_boxed_slice()))
    }

    pub fn rows(&self) -> usize {
        self.rows
    }

    pub fn cols(&self) -> usize {
        self.data.len() / self.rows
    }

    pub fn get(&self, row: usize, col: usize) -> u8 {
        self.data[row * self.cols() + col]
    }

    pub fn get_row(&self, row: usize) -> &[u8] {
        &self.data[row * self.cols()..(row + 1) * self.cols()]
    }

    pub fn get_col(&self, col: usize) -> impl Iterator<Item = u8> + '_ {
        (0..self.rows()).map(move |row| self.get(row, col))
    }

    pub fn get_rows(&self) -> impl Iterator<Item = &[u8]> + '_ {
        (0..self.rows()).map(move |row| self.get_row(row))
    }

    pub fn get_cols(&self) -> impl Iterator<Item = impl Iterator<Item = u8> + '_> + '_ {
        (0..self.cols()).map(move |col| self.get_col(col))
    }

    pub fn get_word(
        &self,
        row: usize,
        col: usize,
        dir: Direction,
        len: usize,
    ) -> Option<impl ExactSizeIterator<Item = u8> + '_> {
        dir.shift_point_bounded((row, col), len - 1, (self.rows(), self.cols()))?;

        Some((0..len).map(move |i| {
            let (row, col) = dir.shift_point((row, col), i);
            self.get(row, col)
        }))
    }

    pub fn set_word(
        &mut self,
        row: usize,
        col: usize,
        dir: Direction,
        word: impl ExactSizeIterator<Item = u8>,
    ) -> bool {
        if dir
            .shift_point_bounded((row, col), word.len() - 1, (self.rows(), self.cols()))
            .is_none()
        {
            return false;
        };

        for (k, ch) in word.enumerate() {
            let (row, col) = dir.shift_point((row, col), k);
            self.data[row * self.cols() + col] = ch;
        }

        true
    }
}

pub trait Solver {
    fn count_occurrences(&self, word: &[u8]) -> usize;
}
