use std::ops::{Range, RangeInclusive};

use crate::util::prelude::*;

fn parse_range(input: &str) -> RangeInclusive<i64> {
    let numbers = input.split('-').map(|x| x.parse().unwrap()).collect_vec();
    numbers[0]..=numbers[1]
}

fn range_contains<T: PartialOrd>(a: &RangeInclusive<T>, b: &RangeInclusive<T>) -> bool {
    a.contains(b.start()) && a.contains(b.end())
}

fn overlaps<T: PartialOrd>(a: &RangeInclusive<T>, b: &RangeInclusive<T>) -> bool {
    a.contains(b.start()) || a.contains(b.end())
}

fn line_one_contains_other(line: &str) -> bool {
    let chunks = line.split(',').map(parse_range).collect_vec();
    range_contains(&chunks[0], &chunks[1]) || range_contains(&chunks[1], &chunks[0])
}

fn line_overlaps(line: &str) -> bool {
    let chunks = line.split(',').map(parse_range).collect_vec();
    overlaps(&chunks[0], &chunks[1]) || range_contains(&chunks[1], &chunks[0])
}

pub fn level1(input: &str) -> i64 {
    input
        .lines()
        .filter(|s| line_one_contains_other(s))
        .count()
        .try_into()
        .unwrap()
}

pub fn level2(input: &str) -> i64 {
    input
        .lines()
        .filter(|s| line_overlaps(s))
        .count()
        .try_into()
        .unwrap()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn level1_given_example() {
        let test_input = include_str!("./test_input/day4.txt");
        assert_eq!(level1(test_input), 2)
    }

    #[test]
    fn level2_given_example() {
        let test_input = include_str!("./test_input/day4.txt");
        assert_eq!(level2(test_input), 4)
    }
}
