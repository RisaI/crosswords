use std::{
    fs::File,
    io::{BufRead, BufReader, BufWriter, Write},
    path::PathBuf,
};

use anyhow::bail;
use clap::Parser;
use crosswords::{hashmap::CrosswordHashMap, needle::CrosswordNeedleSearch, Direction};
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
            let lines = BufReader::new(File::open(input)?).lines();
            let mut data = vec![];

            let mut cols = 0;

            for row in lines {
                let row = row?;

                if row.is_empty() {
                    continue;
                }

                if cols == 0 {
                    cols = row.len();
                }

                if cols != row.len() {
                    bail!("inconsistent row length");
                }

                data.extend(row.as_bytes().iter().copied());
            }

            let crossword = crosswords::Crossword::new(data.len() / cols, data.into_boxed_slice());

            println!(
                "naive: {}",
                crosswords::naive::find_naive(&crossword, word.as_bytes())
            );

            {
                let needle = CrosswordNeedleSearch::new(&crossword);
                println!("needle: {}", needle.find(word.as_bytes()));
            }

            {
                let hash = CrosswordHashMap::<'_, 4>::new(&crossword);
                println!("hash: {}", hash.find(word.as_bytes()));
            }
        }
    }

    Ok(())
}
