use std::{fs::File, io::BufReader};

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use crosswords::{
    hashmap::CrosswordHashMap, naive::NaiveSolver, needle::CrosswordNeedleSearch, Crossword, Solver,
};

pub fn criterion_benchmark(c: &mut Criterion) {
    let crossword = Crossword::parse(BufReader::new(File::open("test.txt").unwrap())).unwrap();

    let words = include_str!("../words.txt")
        .split('\n')
        .map(|w| w.as_bytes())
        .collect::<Vec<_>>();

    let mut group = c.benchmark_group("solvers");

    group.bench_function("naive", |b| {
        let solver = NaiveSolver::new(&crossword);

        b.iter(|| {
            for word in &words {
                solver.count_occurrences(word);
            }
        })
    });

    group.bench_function("needle", |b| {
        let solver = CrosswordNeedleSearch::new(&crossword);

        b.iter(|| {
            for word in &words {
                solver.count_occurrences(word);
            }
        })
    });

    for i in 1..=8 {
        group.bench_with_input(BenchmarkId::new("hash", i), &i, |b, i| {
            let solver = CrosswordHashMap::<'_>::new(&crossword, *i as usize);

            b.iter(|| {
                for word in &words {
                    solver.count_occurrences(word);
                }
            })
        });
    }

    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
