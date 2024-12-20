use ahash::HashMap;
use smallvec::SmallVec;

use crate::{
    utils::{canonical_order, is_palindrome},
    Crossword, Direction, EstimateSize, Solver,
};

type Positions = SmallVec<[(usize, usize, Direction); 2]>;
const STACK_WORD_LEN: usize = 8;

pub struct CrosswordHashMap<'a> {
    word_len: usize,
    crossword: &'a Crossword,
    complete_words: HashMap<SmallVec<[u8; STACK_WORD_LEN]>, usize>,
    incomplete_words: HashMap<SmallVec<[u8; STACK_WORD_LEN]>, Positions>,
}

impl EstimateSize for CrosswordHashMap<'_> {
    fn estimate_size(&self) -> usize {
        self.word_len.estimate_size()
            + std::mem::size_of::<&'_ Crossword>()
            + self.complete_words.estimate_size()
            + self.incomplete_words.estimate_size()
    }
}

impl<'a> CrosswordHashMap<'a> {
    pub fn new(crossword: &'a Crossword, word_len: usize) -> Self {
        assert!(word_len > 0, "non-zero word length required");

        let mut complete_words: HashMap<SmallVec<[u8; STACK_WORD_LEN]>, usize> = HashMap::default();
        let mut incomplete_words: HashMap<SmallVec<[u8; STACK_WORD_LEN]>, Positions> =
            HashMap::default();

        fn add_all_substrings(
            target: &mut HashMap<SmallVec<[u8; STACK_WORD_LEN]>, usize>,
            word: impl Iterator<Item = u8>,
        ) {
            let mut current = SmallVec::<[u8; STACK_WORD_LEN]>::new();

            for next in word {
                current.push(next);

                let canonical = canonical_order(&current);

                if let Some(counter) = target.get_mut(canonical.as_ref()) {
                    *counter += 1;
                } else {
                    target.insert(canonical.iter().copied().collect(), 1);
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

                    let word = word.collect::<SmallVec<[u8; STACK_WORD_LEN]>>();

                    if word_len > 1 || dir == Direction::Right {
                        add_all_substrings(
                            &mut complete_words,
                            word.iter().copied().take(word_len),
                        );
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
}

impl Solver for CrosswordHashMap<'_> {
    fn count_occurrences(&self, word: &[u8]) -> usize {
        if word.len() <= self.word_len {
            return self
                .complete_words
                .get(canonical_order(word).as_ref())
                .copied()
                .unwrap_or_default();
        }

        let incomplete = word
            .iter()
            .copied()
            .take(self.word_len)
            .collect::<SmallVec<[u8; STACK_WORD_LEN]>>();

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
                .collect::<SmallVec<[u8; STACK_WORD_LEN]>>();

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
