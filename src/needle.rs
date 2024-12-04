use std::iter::once;

use memchr::memmem::Finder;
use smallvec::SmallVec;

use crate::Crossword;

const DELIM: u8 = b'.';

pub struct CrosswordNeedleSearch {
    plans: [Box<[u8]>; 4],
}

impl CrosswordNeedleSearch {
    pub fn new(crossword: &Crossword) -> Self {
        let rows = crossword.rows();
        let cols = crossword.cols();

        let direct = crossword
            .get_rows()
            .flat_map(|v| v.iter().copied().chain(once(DELIM)))
            .collect::<Box<[u8]>>();

        let transposed = crossword
            .get_cols()
            .flat_map(|v| v.chain(once(DELIM)))
            .collect::<Box<[u8]>>();

        let diagonal = (0..rows)
            .flat_map(|diag_idx| {
                let start_row = rows - diag_idx - 1;
                let data = &crossword.data;

                (0..(rows - start_row).min(cols))
                    .map(move |j| data[(start_row + j) * cols + j])
                    .chain(once(DELIM))
            })
            .chain((1..cols).flat_map(|start_col| {
                let data = &crossword.data;

                (0..(cols - start_col).min(rows))
                    .map(move |j| data[j * cols + start_col + j])
                    .chain(once(DELIM))
            }))
            .collect::<Box<[u8]>>();

        let anti_diagonal = (0..cols)
            .flat_map(|start_col| {
                let data = &crossword.data;

                (0..(1 + start_col).min(rows))
                    .map(move |j| data[j * cols + start_col - j])
                    .chain(once(DELIM))
            })
            .chain((1..rows).flat_map(|start_row| {
                let data = &crossword.data;

                (0..(rows - start_row).min(cols))
                    .map(move |j| data[(start_row + j) * cols + cols - j - 1])
                    .chain(once(DELIM))
            }))
            .collect::<Box<[u8]>>();

        Self {
            plans: [direct, transposed, diagonal, anti_diagonal],
        }
    }

    pub fn find(&self, needle: &[u8]) -> usize {
        let reverse = needle.iter().rev().copied().collect::<SmallVec<[u8; 16]>>();
        let needles = {
            let mut needles = SmallVec::<[Finder; 2]>::new();
            needles.push(Finder::new(needle));

            if reverse.as_slice() != needle {
                needles.push(Finder::new(&reverse));
            }

            needles
        };

        self.plans
            .iter()
            .flat_map(|v| {
                needles
                    .iter()
                    .map(move |needle| needle.find_iter(v).count())
            })
            .sum::<usize>()
    }
}
