use regex::Regex;

use crate::util::prelude::*;

fn parse_crate_move_line(line: &str) -> Option<(usize, usize, usize)> {
    let regex =
        Regex::new(r#"move (?P<count>\d+) from (?P<source>\d+) to (?P<target>\d+)"#).ok()?;
    let captures = regex.captures(line)?;
    Some((
        captures["count"].parse().ok()?,
        captures["source"].parse().ok()?,
        captures["target"].parse().ok()?,
    ))
}

fn parse_crate_line(line: &str) -> Vec<Option<char>> {
    line.chars()
        .chunks(4)
        .into_iter()
        .map(|mut c| match c.nth(1) {
            Some(' ') => None,
            Some(c) => Some(c),
            None => panic!("Could not parse crate chunk"),
        })
        .collect_vec()
}

pub fn parse_input(input: &str, reverse: bool) -> String {
    let (start_state, moves) = input.split_once("\n 1").unwrap();
    let stack_count = start_state.find('\n').unwrap() / 4 + 1;
    let mut reversed_stacks = vec![Vec::new(); stack_count];

    for line in start_state.lines() {
        for (i, maybe_c) in parse_crate_line(line).into_iter().enumerate() {
            if let Some(c) = maybe_c {
                reversed_stacks[i].push(c)
            }
        }
    }

    let mut stacks = reversed_stacks
        .into_iter()
        .map(|v| v.into_iter().rev().collect_vec())
        .collect_vec();
    for line in moves.lines().skip(2) {
        let (count, source, target) = parse_crate_move_line(line).unwrap();
        let offset = stacks[source - 1].len() - count;
        let mut crates = stacks[source - 1].split_off(offset);
        if reverse {
            crates = crates.into_iter().rev().collect();
        }
        stacks[target - 1].append(&mut crates);
    }
    stacks.into_iter().map(|v| v[v.len() - 1]).collect()
}

pub fn level1(input: &str) -> String {
    parse_input(input, true)
}

pub fn level2(input: &str) -> String {
    parse_input(input, false)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn level1_given_example() {
        let test_input = include_str!("./test_input/day5.txt");
        assert_eq!(level1(test_input), "CMZ")
    }

    #[test]
    fn level2_given_example() {
        let test_input = include_str!("./test_input/day5.txt");
        assert_eq!(level2(test_input), "MCD")
    }
}
