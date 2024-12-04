use std::{
    fs::File,
    io::{BufRead, BufReader},
    iter::once,
};

use memchr::memmem::Finder;

const DELIM: u8 = b'.';

fn main() -> anyhow::Result<()> {
    let data = BufReader::new(File::open("data.txt")?)
        .lines()
        .map(|v| v.map(|v| v.into_bytes()))
        .collect::<Result<Vec<_>, _>>()?;

    let rows = data.len();
    let cols = data[0].len();

    let direct = data
        .iter()
        .flat_map(|v| v.iter().copied().chain(once(DELIM)))
        .collect::<Box<[u8]>>();

    let transposed = (0..cols)
        .flat_map(|col_idx| data.iter().map(move |row| row[col_idx]).chain(once(DELIM)))
        .collect::<Box<[u8]>>();

    let diagonal = (0..rows)
        .flat_map(|diag_idx| {
            let start_row = rows - diag_idx - 1;
            let data = &data;

            (0..(rows - start_row).min(cols))
                .map(move |j| data[start_row + j][j])
                .chain(once(DELIM))
        })
        .chain((1..cols).flat_map(|start_col| {
            let data = &data;

            (0..(cols - start_col).min(rows))
                .map(move |j| data[j][start_col + j])
                .chain(once(DELIM))
        }))
        .collect::<Box<[u8]>>();

    let anti_diagonal = (0..cols)
        .flat_map(|start_col| {
            let data = &data;

            (0..(1 + start_col).min(rows))
                .map(move |j| data[j][start_col - j])
                .chain(once(DELIM))
        })
        .chain((1..rows).flat_map(|start_row| {
            let data = &data;

            (0..(rows - start_row).min(cols))
                .map(move |j| data[start_row + j][cols - j - 1])
                .chain(once(DELIM))
        }))
        .collect::<Box<[u8]>>();

    let directions = [direct, transposed, diagonal, anti_diagonal];

    let needles = [b"XMAS", b"SAMX"].map(Finder::new);
    let mut occurrences = 0;

    for dir in directions {
        for needle in &needles {
            occurrences += needle.find_iter(&dir).count();
        }
    }

    println!("{occurrences} occurrences");

    Ok(())
}
