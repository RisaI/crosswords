use std::{
    fs::File,
    io::{BufReader, BufWriter, Write},
    path::PathBuf,
};

use clap::Parser;
use crosswords::{
    Crossword, CrosswordHashMap, CrosswordNeedleSearch, Direction, NaiveSolver, Solver,
};
use rand::{distributions::Uniform, seq::SliceRandom, Rng};

#[derive(Parser)]
enum Subcommands {
    Generate {
        #[arg(short, long)]
        rows: usize,

        #[arg(short, long)]
        cols: usize,

        #[arg()]
        output: PathBuf,
    },

    Solve {
        #[arg(long)]
        word: String,

        #[arg()]
        input: PathBuf,
    },
}

fn main() -> anyhow::Result<()> {
    let args = Subcommands::parse();

    match args {
        Subcommands::Generate { rows, cols, output } => {
            let rng = &mut rand::thread_rng();
            let mut crosswords = crosswords::Crossword::new(
                rows,
                rng.sample_iter(Uniform::new(b'a', b'z' + 1))
                    .take(rows * cols)
                    .collect::<Box<[u8]>>(),
            );

            let words = include_str!("../words.txt")
                .trim()
                .split('\n')
                .collect::<Vec<_>>();

            for word in words.choose_multiple(rng, rows + cols) {
                loop {
                    let dir = *Direction::ALL.choose(rng).unwrap();
                    let row = rng.gen_range(0..rows);
                    let col = rng.gen_range(0..cols);

                    if crosswords.set_word(row, col, dir, word.as_bytes().iter().copied()) {
                        break;
                    }
                }
            }

            let mut writer = BufWriter::new(File::create(output)?);

            for (idx, row) in crosswords.get_rows().enumerate() {
                if idx > 0 {
                    writer.write_all(b"\n")?;
                }

                writer.write_all(row)?;
            }
        }
        Subcommands::Solve { word, input } => {
            let crossword = Crossword::parse(BufReader::new(File::open(input)?))?;

            println!(
                "naive: {}",
                NaiveSolver::new(&crossword).count_occurrences(word.as_bytes())
            );

            {
                let needle = CrosswordNeedleSearch::new(&crossword);
                println!("needle: {}", needle.count_occurrences(word.as_bytes()));
            }

            {
                let hash = CrosswordHashMap::<'_>::new(&crossword, 4);
                println!("hash4: {}", hash.count_occurrences(word.as_bytes()));
            }
        }
    }

    Ok(())
}
