use std::iter::once;

use fxhash::FxHashMap as HashMap;

use crate::{utils::is_palindrome, Crossword, Direction, Solver};

#[derive(Default)]
pub struct TrieEntry {
    count: usize,
    children: HashMap<u8, TrieEntry>,
}

impl TrieEntry {
    pub fn incr(&mut self) {
        self.count += 1;
    }

    pub fn decr(&mut self) {
        self.count = self.count.saturating_sub(1);
    }

    pub fn insert(&mut self, mut word: impl Iterator<Item = u8>) {
        let Some(ch) = word.next() else {
            return;
        };

        let child = self.children.entry(ch).or_default();

        child.incr();
        child.insert(word);
    }

    pub fn count_occurrences(&self, word: &[u8]) -> usize {
        if word.is_empty() {
            return self.count;
        }

        match self.children.get(&word[0]) {
            Some(child) => child.count_occurrences(&word[1..]),
            None => 0,
        }
    }
}

pub struct Trie {
    root: TrieEntry,
}

impl Trie {
    pub fn new(crossword: &Crossword, word_len_limit: Option<usize>) -> Self {
        let mut root = TrieEntry::default();

        let max_len = word_len_limit.unwrap_or(crossword.rows().max(crossword.cols()));

        for row in 0..crossword.rows() {
            for col in 0..crossword.cols() {
                let mut valid_dirs = 0;
                let central_char = crossword.get(row, col);

                for dir in Direction::ALL {
                    for len in (2..=max_len).rev() {
                        let Some(word) = crossword.get_word(row, col, dir, len) else {
                            continue;
                        };

                        valid_dirs += 1;
                        root.insert(word);
                        break;
                    }
                }

                // Prevent central character from being added more than once
                if valid_dirs > 1 {
                    for _ in 0..(valid_dirs - 1) {
                        root.children.get_mut(&central_char).unwrap().decr();
                    }
                } else if valid_dirs == 0 {
                    root.insert(once(central_char));
                }
            }
        }

        Self { root }
    }
}

impl Solver for Trie {
    fn count_occurrences(&self, word: &[u8]) -> usize {
        if word.is_empty() {
            return 0;
        }

        if is_palindrome(word) {
            self.root.count_occurrences(word)
        } else {
            let reverse = word.iter().rev().copied().collect::<Vec<_>>();

            self.root.count_occurrences(word) + self.root.count_occurrences(&reverse)
        }
    }
}
