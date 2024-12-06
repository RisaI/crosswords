use std::{fs::File, io::BufReader};

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use crosswords::{Crossword, CrosswordHashMap, CrosswordNeedleSearch, Solver, Trie};

pub fn criterion_benchmark(c: &mut Criterion) {
    let crossword = Crossword::parse(BufReader::new(File::open("test_64k.txt").unwrap())).unwrap();

    let words = include_str!("../words.txt")
        .split('\n')
        .map(|w| w.as_bytes())
        .collect::<Vec<_>>();

    let mut group = c.benchmark_group("solvers");

    let needle_solver = CrosswordNeedleSearch::new(&crossword);
    let trie_solver = Trie::new(&crossword, Some(14));

    // group.sample_size(20);

    // group
    //     .bench_function("naive", |b| {
    //         let solver = NaiveSolver::new(&crossword);

    //         b.iter(|| {
    //             for word in &words {
    //                 solver.count_occurrences(word);
    //             }
    //         })
    //     });

    group.bench_function("needle", |b| {
        b.iter(|| {
            for word in &words {
                needle_solver.count_occurrences(word);
            }
        })
    });

    group.bench_function("trie", |b| {
        b.iter(|| {
            for word in &words {
                trie_solver.count_occurrences(word);
            }
        })
    });

    for i in 1..=8 {
        let hash_solver = CrosswordHashMap::<'_>::new(&crossword, i as usize);

        group.bench_with_input(BenchmarkId::new("hash", i), &i, |b, _| {
            b.iter(|| {
                for word in &words {
                    hash_solver.count_occurrences(word);
                }
            })
        });
    }

    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
