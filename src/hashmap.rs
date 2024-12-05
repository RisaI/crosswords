use std::collections::HashMap;

use ahash::RandomState;
use smallvec::SmallVec;

use crate::{utils::is_palindrome, Crossword, Direction, Solver};

type Positions = SmallVec<[(usize, usize, Direction); 2]>;

pub struct CrosswordHashMap<'a> {
    word_len: usize,
    crossword: &'a Crossword,
    complete_words: HashMap<SmallVec<[u8; 16]>, usize, RandomState>,
    incomplete_words: HashMap<SmallVec<[u8; 16]>, Positions, RandomState>,
}

impl<'a> CrosswordHashMap<'a> {
    pub fn new(crossword: &'a Crossword, word_len: usize) -> Self {
        assert!(word_len > 0, "non-zero word length required");

        let mut complete_words: HashMap<SmallVec<[u8; 16]>, usize, RandomState> =
            HashMap::default();
        let mut incomplete_words: HashMap<SmallVec<[u8; 16]>, Positions, RandomState> =
            HashMap::default();

        fn add_all_substrings(
            target: &mut HashMap<SmallVec<[u8; 16]>, usize, RandomState>,
            word: impl Iterator<Item = u8>,
        ) {
            let mut current = SmallVec::<[u8; 16]>::new();

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
                    let Some(word) = crossword.get_word(row, col, dir, word_len) else {
                        for len in (1..word_len).rev() {
                            if let Some(found) = crossword.get_word(row, col, dir, len) {
                                add_all_substrings(&mut complete_words, found);
                                break;
                            }
                        }

                        continue;
                    };

                    let word = word.collect::<SmallVec<[u8; 16]>>();

                    if word_len > 1 || dir == Direction::Right {
                        add_all_substrings(&mut complete_words, word.iter().copied());
                    }

                    incomplete_words
                        .entry(word)
                        .or_default()
                        .push((row, col, dir));
                }
            }
        }

        Self {
            word_len,
            crossword,
            complete_words,
            incomplete_words,
        }
    }

    pub fn estimate_size(&self) -> usize {
        self.complete_words.len() * (16 + 1)
            + self.incomplete_words.len() * 16
            + self
                .incomplete_words
                .values()
                .map(|p| p.len() * (size_of::<(usize, usize, Direction)>()))
                .sum::<usize>()
    }
}

impl Solver for CrosswordHashMap<'_> {
    fn count_occurrences(&self, word: &[u8]) -> usize {
        if word.len() <= self.word_len {
            return self.complete_words.get(word).copied().unwrap_or_default()
                + if !is_palindrome(word) {
                    let reverse = word.iter().rev().copied().collect::<SmallVec<[u8; 16]>>();

                    self.complete_words
                        .get(&reverse)
                        .copied()
                        .unwrap_or_default()
                } else {
                    0
                };
        }

        let incomplete = word
            .iter()
            .copied()
            .take(self.word_len)
            .collect::<SmallVec<[u8; 16]>>();

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

        if !is_palindrome(word) {
            let reverse = word
                .iter()
                .copied()
                .rev()
                .take(self.word_len)
                .collect::<SmallVec<[u8; 16]>>();

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
