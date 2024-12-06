use std::{borrow::Cow, cmp::Ordering};

pub fn is_palindrome(word: &[u8]) -> bool {
    let len = word.len() / 2;
    word.iter().take(len).eq(word.iter().rev().take(len))
}

pub fn canonical_order(word: &[u8]) -> Cow<'_, [u8]> {
    let len = word.len() / 2;
    let reverse = word
        .iter()
        .take(len)
        .zip(word.iter().rev().take(len))
        .filter_map(|(a, b)| match a.cmp(b) {
            std::cmp::Ordering::Equal => None,
            v => Some(v),
        })
        .next();

    if let Some(Ordering::Greater) = reverse {
        Cow::Owned(word.iter().rev().copied().collect::<Vec<_>>())
    } else {
        Cow::Borrowed(word)
    }
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
