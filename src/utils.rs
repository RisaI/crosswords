pub fn is_palindrome(word: &[u8]) -> bool {
    let len = word.len() / 2;
    word.iter().take(len).eq(word.iter().rev().take(len))
}

pub fn many_iter_eq<T: PartialEq, const N: usize>(
    mut pivot: impl Iterator<Item = T>,
    mut iters: [&mut dyn Iterator<Item = T>; N],
) -> [bool; N] {
    let mut eqs = [true; N];

    'outer: loop {
        let pivot = pivot.next();

        for (i, iter) in iters.iter_mut().enumerate() {
            // Don't poll known non-equal values
            if !eqs[i] {
                continue;
            }

            if pivot != iter.next() {
                eqs[i] = false;

                // Terminate if none is equal
                if eqs.iter().all(|v| !v) {
                    break 'outer;
                }
            }
        }

        if pivot.is_none() {
            break;
        }
    }

    eqs
}
