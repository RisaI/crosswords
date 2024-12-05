use core::str;
use std::{fs::File, io::BufReader};

use crosswords::{naive::NaiveSolver, needle::CrosswordNeedleSearch, Crossword, Solver};

#[test]
fn solver_output_matches() {
    let crossword = Crossword::parse(BufReader::new(File::open("test.txt").unwrap())).unwrap();
    let words = include_str!("../words.txt")
        .split('\n')
        .filter(|w| !w.is_empty())
        .map(|w| w.as_bytes())
        .collect::<Vec<_>>();

    let naive = NaiveSolver::new(&crossword);
    let mut solvers: Vec<Box<dyn Solver>> = vec![
        Box::new(NaiveSolver::new(&crossword)),
        Box::new(CrosswordNeedleSearch::new(&crossword)),
    ];

    (1..=8).for_each(|i| {
        solvers.push(Box::new(crosswords::hashmap::CrosswordHashMap::<'_>::new(
            &crossword, i,
        )))
    });

    for word in words {
        let naive_count = naive.count_occurrences(word);

        for (idx, solver) in solvers.iter().enumerate() {
            let count = solver.count_occurrences(word);

            assert_eq!(
                naive_count,
                count,
                "occurrences of '{}' should match across solvers, mismatch for {}",
                unsafe { str::from_utf8_unchecked(word) },
                match idx {
                    0 => "naive".into(),
                    1 => "needle".into(),
                    v => format!("hash{}", v - 1),
                }
            );
        }
    }
}
