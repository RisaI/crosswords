use std::collections::HashMap;

use smallvec::SmallVec;

use crate::{Crossword, Direction};

type Positions = SmallVec<[(usize, usize, Direction); 2]>;

pub struct CrosswordHashMap<'a, const MAX_WORD_LENGTH: usize> {
    crossword: &'a Crossword,
    complete_words: HashMap<SmallVec<[u8; MAX_WORD_LENGTH]>, usize>,
    incomplete_words: HashMap<[u8; MAX_WORD_LENGTH], Positions>,
}

impl<'a, const MAX_WORD_LENGTH: usize> CrosswordHashMap<'a, MAX_WORD_LENGTH> {
    pub fn new(crossword: &'a Crossword) -> Self {
        let mut complete_words = HashMap::new();
        let mut incomplete_words: HashMap<[u8; MAX_WORD_LENGTH], Positions> = HashMap::new();

        fn add_all_substrings<const MAX_WORD_LENGTH: usize>(
            target: &mut HashMap<SmallVec<[u8; MAX_WORD_LENGTH]>, usize>,
            word: impl Iterator<Item = u8>,
        ) {
            let mut current = SmallVec::<[u8; MAX_WORD_LENGTH]>::new();

            for next in word {
                current.push(next);

                if target.contains_key(&current) {
                    *target.get_mut(&current).unwrap() += 1;
                } else {
                    target.insert(current.clone(), 1);
                }
            }
        }

        for row in 0..crossword.rows() {
            for col in 0..crossword.cols() {
                for dir in Direction::ALL {
                    let Some(mut word) = crossword.get_word(row, col, dir, MAX_WORD_LENGTH) else {
                        for len in (1..MAX_WORD_LENGTH).rev() {
                            if let Some(found) = crossword.get_word(row, col, dir, len) {
                                add_all_substrings(&mut complete_words, found);
                                break;
                            }
                        }

                        continue;
                    };

                    let word = std::array::from_fn(|_| word.next().unwrap());

                    add_all_substrings(&mut complete_words, word.iter().copied());
                    incomplete_words
                        .entry(word)
                        .or_default()
                        .push((row, col, dir));
                }
            }
        }

        Self {
            crossword,
            complete_words,
            incomplete_words,
        }
    }

    pub fn estimate_size(&self) -> usize {
        self.complete_words.len() * (MAX_WORD_LENGTH + 1)
            + self.incomplete_words.len() * MAX_WORD_LENGTH
            + self
                .incomplete_words
                .values()
                .map(|p| p.len() * (size_of::<(usize, usize, Direction)>()))
                .sum::<usize>()
    }

    pub fn find(&self, word: &[u8]) -> usize {
        if word.len() <= MAX_WORD_LENGTH {
            let reverse = word
                .iter()
                .rev()
                .copied()
                .collect::<SmallVec<[u8; MAX_WORD_LENGTH]>>();

            return self.complete_words.get(word).copied().unwrap_or(0)
                + if reverse.as_slice() != word {
                    self.complete_words.get(&reverse).copied().unwrap_or(0)
                } else {
                    0
                };
        }

        let incomplete = std::array::from_fn(|i| word[i]);
        let reverse = std::array::from_fn(|i| word[word.len() - i - 1]);

        let mut count = 0;

        if let Some(directions) = self.incomplete_words.get(&incomplete) {
            for &(row, col, dir) in directions {
                if let Some(found) = self.crossword.get_word(row, col, dir, word.len()) {
                    if found.eq(word.iter().copied()) {
                        count += 1;
                    }
                }
            }
        }

        if reverse != incomplete {
            if let Some(directions) = self.incomplete_words.get(&reverse) {
                for &(row, col, dir) in directions {
                    if let Some(found) = self.crossword.get_word(row, col, dir, word.len()) {
                        if found.eq(word.iter().rev().copied()) {
                            count += 1;
                        }
                    }
                }
            }
        }

        count
    }
}
