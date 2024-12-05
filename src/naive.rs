use crate::{utils::many_iter_eq, Crossword, Direction, Solver};

pub struct NaiveSolver<'a>(&'a Crossword);

impl<'a> NaiveSolver<'a> {
    pub fn new(crossword: &'a Crossword) -> Self {
        Self(crossword)
    }
}

impl Solver for NaiveSolver<'_> {
    fn count_occurrences(&self, word: &[u8]) -> usize {
        let crossword = &self.0;
        let mut occurrences = 0;

        for row in 0..crossword.rows() {
            for col in 0..crossword.cols() {
                for dir in Direction::ALL {
                    let Some(found) = crossword.get_word(row, col, dir, word.len()) else {
                        continue;
                    };

                    if many_iter_eq(
                        found,
                        [&mut word.iter().copied(), &mut word.iter().rev().copied()],
                    )
                    .into_iter()
                    .any(|v| v)
                    {
                        occurrences += 1;
                    }
                }
            }
        }

        occurrences
    }
}
