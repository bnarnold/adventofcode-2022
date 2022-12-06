use std::collections::HashSet;

use crate::util::prelude::*;
use bitvec::prelude::*;

type CharMask = BitArr!(for 26, in u32);
#[derive(Debug)]
enum CharCounter {
    FoundDuplicate,
    Seen(CharMask),
}

impl FromIterator<char> for CharCounter {
    fn from_iter<T: IntoIterator<Item = char>>(iter: T) -> Self {
        let mut char_mask: CharMask = BitArray::ZERO;
        for c in iter {
            let i = (ascii_code(c) - LOWER_A_ASCII) as usize;
            if char_mask[i] {
                return CharCounter::FoundDuplicate;
            }
            let mut bit_ref = char_mask.get_mut(i).unwrap();
            *bit_ref = true;
        }
        CharCounter::Seen(char_mask)
    }
}

impl CharCounter {
    fn result(&self) -> bool {
        match self {
            CharCounter::FoundDuplicate => false,
            CharCounter::Seen(_) => true,
        }
    }
}

pub fn first_distinct_chunk(input: &str, size: usize) -> usize {
    input
        .chars()
        .collect_vec()
        .windows(size)
        .enumerate()
        .find(|(_, w)| w.iter().copied().collect::<CharCounter>().result())
        .unwrap()
        .0
        + size
}

pub fn level1(input: &str) -> usize {
    first_distinct_chunk(input, 4)
}

pub fn level2(input: &str) -> usize {
    first_distinct_chunk(input, 14)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn level1_given_example() {
        let test_input = include_str!("./test_input/day6.txt");
        assert_eq!(level1(test_input), 7)
    }

    #[test]
    fn level2_given_example() {
        let test_input = include_str!("./test_input/day6.txt");
        assert_eq!(level2(test_input), 19)
    }
}
