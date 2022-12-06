use std::collections::HashSet;

use crate::util::prelude::*;

pub fn first_distinct_chunk(input: &str, size: usize) -> usize {
    input
        .chars()
        .collect_vec()
        .windows(size)
        .enumerate()
        .find(|(_, w)| w.iter().copied().collect::<HashSet<_>>().len() == size)
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
