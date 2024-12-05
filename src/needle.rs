use std::iter::once;

use memchr::memmem::Finder;
use smallvec::SmallVec;

use crate::{utils::is_palindrome, Crossword, Solver};

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
}

impl Solver for CrosswordNeedleSearch {
    fn count_occurrences(&self, word: &[u8]) -> usize {
        let reverse = word.iter().rev().copied().collect::<SmallVec<[u8; 16]>>();
        let needles = {
            let mut needles = SmallVec::<[Finder; 2]>::new();
            needles.push(Finder::new(word));

            if !is_palindrome(word) {
                needles.push(Finder::new(&reverse));
            }

            needles
        };

        self.plans
            .iter()
            .flat_map(|plan| {
                needles
                    .iter()
                    .map(move |needle| needle.find_iter(plan).count())
            })
            .sum::<usize>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn transposed_shapes() {
        let crossword = Crossword::new(3, b"abcdefghi".to_vec().into_boxed_slice());
        let needle = CrosswordNeedleSearch::new(&crossword);

        assert_eq!(needle.plans[0].as_ref(), b"abc.def.ghi.");
        assert_eq!(needle.plans[1].as_ref(), b"adg.beh.cfi.");
        assert_eq!(needle.plans[2].as_ref(), b"g.dh.aei.bf.c.");
        assert_eq!(needle.plans[3].as_ref(), b"a.bd.ceg.fh.i.");
    }
}
