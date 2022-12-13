use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{i32, line_ending},
    combinator::{flat_map, map, value},
    multi::separated_list0,
    sequence::preceded,
    IResult,
};

use crate::util::prelude::*;

#[derive(Debug, Clone)]
enum Op {
    Noop,
    Addx(i32),
}

fn parse_line(line: &str) -> IResult<&str, Op> {
    alt((
        value(Op::Noop, tag("noop")),
        map(preceded(tag("addx "), i32), Op::Addx),
    ))(line)
}

fn parse_input(input: &str) -> IResult<&str, Vec<(usize, i32)>> {
    let mut step = 1;
    let mut x = 1;
    map(
        separated_list0(
            line_ending,
            map(parse_line, move |op| match op {
                Op::Noop => {
                    let result = vec![(step, x)];
                    step += 1;
                    result
                }
                Op::Addx(y) => {
                    let result = vec![(step, x), (step + 1, x)];
                    step += 2;
                    x += y;
                    result
                }
            }),
        ),
        |acc| acc.into_iter().flatten().collect(),
    )(input)
}

pub fn level1(input: &str) -> i32 {
    parse_input(input)
        .unwrap()
        .1
        .into_iter()
        .filter_map(|(i, x)| (i % 40 == 20).then_some((i as i32) * x))
        .take(6)
        .sum()
}

pub fn level2(input: &str) -> i32 {
    let chars = parse_input(input)
        .unwrap()
        .1
        .into_iter()
        .map(|(i, x)| {
            if ((i as i32 - 1).rem_euclid(40) - x).abs() <= 1 {
                '■'
            } else {
                ' '
            }
        })
        .collect_vec();
    for line in chars.chunks(40) {
        let line: String = line.iter().collect();
        println!("{line}");
    }
    0
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn level1_given_example() {
        let test_input = include_str!("./test_input/day10.txt");
        assert_eq!(level1(test_input), 13140)
    }

    #[test]
    fn level2_given_example() {
        let test_input = include_str!("./test_input/day10.txt");
        assert_eq!(level2(test_input), 0)
    }
}
