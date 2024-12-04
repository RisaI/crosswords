use crate::Crossword;

pub struct CrosswordHashMap<'a> {
    crossword: &'a Crossword,
    map: Box<[u8]>,
}
