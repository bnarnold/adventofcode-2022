use std::{any, cmp::Ordering, collections::HashSet};

use nom::{
    character::complete::{anychar, char, i64, line_ending},
    combinator::{map, map_res},
    multi::separated_list0,
    sequence::separated_pair,
    Finish, IResult, Parser,
};

use crate::util::prelude::*;

#[derive(Debug, Copy, Clone)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl TryFrom<char> for Direction {
    type Error = anyhow::Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        use Direction::*;
        match value {
            'L' => Ok(Left),
            'R' => Ok(Right),
            'U' => Ok(Up),
            'D' => Ok(Down),
            _ => Err(anyhow!("unknown direction char")),
        }
    }
}

#[derive(Debug)]
struct Move {
    length: i64,
    direction: Direction,
}

fn direction_line(line: &str) -> IResult<&str, Move> {
    map(
        separated_pair(map_res(anychar, |c| c.try_into()), char(' '), i64),
        |(direction, length)| Move { direction, length },
    )(line)
}

fn parse_input(input: &str) -> IResult<&str, Vec<Move>> {
    separated_list0(line_ending, direction_line)(input)
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, Default)]
struct Pos(i64, i64);

impl Pos {
    fn move_to_head(&mut self, head_pos: &Pos) {
        let Pos(head_x, head_y) = head_pos;
        if (head_x - self.0).abs() > 1 || (head_y - self.1).abs() > 1 {
            self.0 += (head_x - self.0).signum();
            self.1 += (head_y - self.1).signum();
        }
    }

    fn step(&mut self, direction: &Direction) {
        match direction {
            Direction::Left => self.0 -= 1,
            Direction::Right => self.0 += 1,
            Direction::Up => self.1 += 1,
            Direction::Down => self.1 -= 1,
        }
    }
}

#[derive(Debug)]
struct Rope<const N: usize> {
    nodes: [Pos; N],
}

impl<const N: usize> Rope<N> {
    fn new() -> Self {
        Rope {
            nodes: [Pos(0, 0); N],
        }
    }

    fn step(&mut self, direction: &Direction) {
        self.nodes[0].step(direction);
        let mut last = self.nodes[0];
        for node in self.nodes.iter_mut().skip(1) {
            node.move_to_head(&last);
            last = *node
        }
    }

    fn tail(&self) -> Pos {
        self.nodes[N - 1]
    }
}

fn move_rope<'a, const N: usize>(moves: impl Iterator<Item = &'a Move>) -> usize {
    let mut rope: Rope<N> = Rope::new();
    let mut seen = HashSet::new();
    seen.insert(rope.tail());
    for Move { direction, length } in moves {
        for _ in 0..*length {
            rope.step(direction);
            seen.insert(rope.tail());
        }
    }
    seen.len()
}

pub fn level1(input: &str) -> usize {
    let moves = parse_input(input).finish().unwrap().1;
    move_rope::<'_, 2>(moves.iter())
}

pub fn level2(input: &str) -> usize {
    let moves = parse_input(input).finish().unwrap().1;
    move_rope::<'_, 10>(moves.iter())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn level1_given_example() {
        let test_input = include_str!("./test_input/day9.txt");
        assert_eq!(level1(test_input), 13)
    }

    #[test]
    fn level2_given_example() {
        let test_input = include_str!("./test_input/day9.txt");
        assert_eq!(level2(test_input), 1)
    }

    #[test]
    fn level2_larger_example() {
        let test_input = include_str!("./test_input/day9_large.txt");
        assert_eq!(level2(test_input), 36)
    }
}
