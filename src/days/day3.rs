use std::collections::HashSet;

use crate::util::prelude::*;

fn priority(c: char) -> i64 {
    if ('a'..='z').contains(&c) {
        ascii_code(c) - LOWER_A_ASCII + 1
    } else if ('A'..='Z').contains(&c) {
        ascii_code(c) - UPPER_A_ASCII + 27
    } else {
        panic!("Can only handle non-extended alphabet characters")
    }
}

fn parse_line(line: &str) -> i64 {
    let (first, second) = line.split_at(line.len() / 2);
    let first_set: HashSet<_> = first.chars().collect();
    let second_set: HashSet<_> = second.chars().collect();
    priority(*first_set.intersection(&second_set).next().unwrap())
}

fn parse_line_group<'a>(lines: impl Iterator<Item = &'a str>) -> i64 {
    let char_in_all = lines
        .map(|line| line.chars().collect::<HashSet<_>>())
        .reduce(|a, b| a.intersection(&b).copied().collect())
        .unwrap()
        .into_iter()
        .next()
        .unwrap();
    priority(char_in_all)
}

pub fn level1(input: &str) -> i64 {
    input.lines().map(parse_line).sum()
}

pub fn level2(input: &str) -> i64 {
    input
        .lines()
        .chunks(3)
        .into_iter()
        .map(parse_line_group)
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn level1_given_example() {
        let test_input = include_str!("./test_input/day3.txt");
        assert_eq!(level1(test_input), 157)
    }

    #[test]
    fn level2_given_example() {
        let test_input = include_str!("./test_input/day3.txt");
        assert_eq!(level2(test_input), 70)
    }
}
