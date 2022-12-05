use nom::{
    branch::alt,
    bytes::complete::tag,
    character::{
        complete::{char, newline, satisfy, u32},
        is_alphabetic,
    },
    combinator::{map, value},
    multi::{count, many1, separated_list1},
    sequence::{delimited, pair, preceded},
    IResult, ToUsize,
};

use crate::util::prelude::*;

#[derive(Debug)]
struct Move {
    count: usize,
    source: usize,
    target: usize,
}
#[derive(Debug)]
struct CrateMoves {
    crate_lines: Vec<Vec<Option<char>>>,
    moves: Vec<Move>,
}

fn parse_crate_chunk(chunk: &str) -> IResult<&str, Option<char>> {
    alt((
        map(
            delimited(char('['), satisfy(|c| c.is_alphanumeric()), char(']')),
            Some,
        ),
        value(None, tag("   ")),
    ))(chunk)
}
fn parse_crate_line(line: &str) -> IResult<&str, Vec<Option<char>>> {
    separated_list1(char(' '), parse_crate_chunk)(line)
}
fn parse_digit_line(line: &str) -> IResult<&str, ()> {
    value(
        (),
        delimited(
            char(' '),
            separated_list1(tag("   "), satisfy(|c| c.is_ascii_digit())),
            char(' '),
        ),
    )(line)
}
fn parse_usize(input: &str) -> IResult<&str, usize> {
    map(u32, |x| x.to_usize())(input)
}
fn parse_crate_move_line(line: &str) -> IResult<&str, Move> {
    let (line, _) = tag("move ")(line)?;
    let (line, count) = parse_usize(line)?;
    let (line, _) = tag(" from ")(line)?;
    let (line, source) = parse_usize(line)?;
    let (line, _) = tag(" to ")(line)?;
    let (output, target) = parse_usize(line)?;
    IResult::Ok((
        output,
        Move {
            count,
            source: source - 1,
            target: target - 1,
        },
    ))
}

fn parse_input(input: &str) -> IResult<&str, CrateMoves> {
    let (input, crate_lines) = separated_list1(newline, parse_crate_line)(input)?;
    let (input, _) = newline(input)?;
    let (input, _) = parse_digit_line(input)?;
    let (input, _) = count(newline, 2)(input)?;
    let (output, moves) = separated_list1(newline, parse_crate_move_line)(input)?;
    IResult::Ok((output, CrateMoves { crate_lines, moves }))
}

fn move_crates(crate_moves: CrateMoves, reverse: bool) -> String {
    let mut reversed_stacks: Vec<Vec<char>> =
        vec![Vec::with_capacity(crate_moves.crate_lines.len()); crate_moves.crate_lines[0].len()];

    for line in crate_moves.crate_lines {
        for (i, maybe_c) in line.iter().enumerate() {
            if let Some(c) = maybe_c {
                reversed_stacks[i].push(*c)
            }
        }
    }

    let mut stacks = reversed_stacks
        .into_iter()
        .map(|v| v.into_iter().rev().collect_vec())
        .collect_vec();
    for Move {
        count,
        source,
        target,
    } in crate_moves.moves
    {
        let offset = stacks[source].len() - count;
        let mut crates = stacks[source].split_off(offset);
        if reverse {
            crates = crates.into_iter().rev().collect();
        }
        stacks[target].append(&mut crates);
    }
    stacks.into_iter().map(|v| v[v.len() - 1]).collect()
}

pub fn level1(input: &str) -> String {
    let (_, crate_moves) = parse_input(input).unwrap();
    move_crates(crate_moves, true)
}

pub fn level2(input: &str) -> String {
    let (_, crate_moves) = parse_input(input).unwrap();
    move_crates(crate_moves, false)
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
