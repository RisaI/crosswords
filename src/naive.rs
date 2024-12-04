use crate::{Crossword, Direction};

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

pub fn find_naive(crossword: &Crossword, needle: &[u8]) -> usize {
    let mut occurrences = 0;

    for row in 0..crossword.rows() {
        for col in 0..crossword.cols() {
            for dir in Direction::ALL {
                let Some(word) = crossword.get_word(row, col, dir, needle.len()) else {
                    continue;
                };

                if many_iter_eq(
                    word,
                    [
                        &mut needle.iter().copied(),
                        &mut needle.iter().rev().copied(),
                    ],
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
