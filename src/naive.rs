use crate::{Crossword, Direction, Solver};

pub fn many_iter_eq<T: PartialEq, const N: usize>(
    mut pivot: impl Iterator<Item = T>,
    mut iters: [&mut dyn Iterator<Item = T>; N],
) -> [bool; N] {
    let mut eqs = [true; N];

    'outer: loop {
        let pivot = pivot.next();

        for (i, iter) in iters.iter_mut().enumerate() {
            // Don't poll known non-equal values
            if !eqs[i] {
                continue;
            }

            if pivot != iter.next() {
                eqs[i] = false;

                // Terminate if none is equal
                if eqs.iter().all(|v| !v) {
                    break 'outer;
                }
            }
        }

        if pivot.is_none() {
            break;
        }
    }

    eqs
}

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
