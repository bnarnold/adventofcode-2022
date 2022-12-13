use itertools::Either;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, line_ending, u16, u64},
    combinator::{map, map_opt, value},
    multi::{count, separated_list0},
    sequence::{delimited, preceded, tuple},
    IResult,
};

use crate::util::prelude::*;

#[derive(Debug, Clone)]
enum Var {
    Lit(u64),
    Old,
}

impl Var {
    fn apply(&self, x: u64) -> u64 {
        match self {
            Var::Old => x,
            Var::Lit(y) => *y,
        }
    }
}

#[derive(Debug, Clone)]
enum Op {
    Add,
    Mul,
}

#[derive(Debug)]
struct Formula {
    left: Var,
    right: Var,
    op: Op,
}

impl Formula {
    fn apply(&self, x: u64) -> u64 {
        let left = self.left.apply(x);
        let right = self.right.apply(x);
        match self.op {
            Op::Add => left + right,
            Op::Mul => left * right,
        }
    }
}

fn var(input: &str) -> IResult<&str, Var> {
    alt((value(Var::Old, tag("old")), map(u64, Var::Lit)))(input)
}

fn op(input: &str) -> IResult<&str, Op> {
    alt((value(Op::Add, char('+')), value(Op::Mul, char('*'))))(input)
}

fn monkey_pos(line: &str) -> IResult<&str, usize> {
    delimited(tag("Monkey "), map(u16, Into::into), char(':'))(line)
}

fn starting_items(line: &str) -> IResult<&str, Vec<u64>> {
    preceded(tag("  Starting items: "), separated_list0(tag(", "), u64))(line)
}

fn formula(line: &str) -> IResult<&str, Formula> {
    preceded(
        tag("  Operation: new = "),
        map(
            tuple((var, delimited(char(' '), op, char(' ')), var)),
            |(left, op, right)| Formula { left, right, op },
        ),
    )(line)
}

fn test_divisible(line: &str) -> IResult<&str, u64> {
    preceded(tag("  Test: divisible by "), u64)(line)
}

fn throw(test_success: bool) -> impl FnMut(&str) -> IResult<&str, usize> {
    move |line| {
        preceded(
            tuple((
                tag("    If "),
                tag(if test_success { "true" } else { "false" }),
                tag(": throw to monkey "),
            )),
            map(u16, Into::into),
        )(line)
    }
}

#[derive(Debug)]
struct Monkey {
    items: Vec<u64>,
    formula: Formula,
    test_divisible: u64,
    throw_to_true: usize,
    throw_to_false: usize,
    inspected: usize,
}

impl Monkey {
    fn inspect_item(&self, size_bound: &SizeBound, x: u64) -> Either<u64, u64> {
        let result = size_bound.apply(self.formula.apply(x));
        if result % self.test_divisible == 0 {
            Either::Left(result)
        } else {
            Either::Right(result)
        }
    }
    fn inspect(&mut self, size_bound: &SizeBound) -> (Vec<u64>, Vec<u64>) {
        self.inspected += self.items.len();
        std::mem::take(&mut self.items)
            .into_iter()
            .partition_map(|x| self.inspect_item(size_bound, x))
    }
}

fn monkey(input: &str) -> IResult<&str, (usize, Monkey)> {
    map(
        tuple((
            monkey_pos,
            preceded(line_ending, starting_items),
            preceded(line_ending, formula),
            preceded(line_ending, test_divisible),
            preceded(line_ending, throw(true)),
            preceded(line_ending, throw(false)),
        )),
        |(i, items, formula, test_divisible, throw_to_true, throw_to_false)| {
            (
                i,
                Monkey {
                    items,
                    formula,
                    test_divisible,
                    throw_to_true,
                    throw_to_false,
                    inspected: 0,
                },
            )
        },
    )(input)
}

enum SizeBound {
    DivideBy(u64),
    Modulus(u64),
}

impl SizeBound {
    fn apply(&self, x: u64) -> u64 {
        match self {
            SizeBound::DivideBy(d) => x / d,
            SizeBound::Modulus(d) => x % d,
        }
    }
}

struct MonkeyCabal {
    monkeys: Vec<Monkey>,
    size_bound: SizeBound,
}

impl MonkeyCabal {
    fn monkey_business(&self) -> usize {
        self.monkeys
            .iter()
            .map(|monkey| monkey.inspected)
            .sorted_by(|x, y| x.cmp(y).reverse())
            .take(2)
            .product()
    }

    fn round(&mut self) {
        for i in 0..self.monkeys.len() {
            let (mut passed, mut failed) = self.monkeys[i].inspect(&self.size_bound);
            let Monkey {
                throw_to_true,
                throw_to_false,
                ..
            } = self.monkeys[i];
            self.monkeys[throw_to_true].items.append(&mut passed);
            self.monkeys[throw_to_false].items.append(&mut failed);
        }
    }
}

fn gcd(a: u64, b: u64) -> u64 {
    let mut x = a;
    let mut y = b;
    while y != 0 {
        let z = x.rem_euclid(y);
        x = y;
        y = z;
    }
    x
}

fn lcm(a: u64, b: u64) -> u64 {
    a * (b / gcd(a, b))
}

impl From<(Option<u64>, Vec<Monkey>)> for MonkeyCabal {
    fn from(value: (Option<u64>, Vec<Monkey>)) -> Self {
        let size_bound = match value.0 {
            Some(d) => SizeBound::DivideBy(d),
            None => SizeBound::Modulus(
                value
                    .1
                    .iter()
                    .fold(1, |acc, monkey| lcm(acc, monkey.test_divisible)),
            ),
        };
        MonkeyCabal {
            monkeys: value.1,
            size_bound,
        }
    }
}

fn parse_input(size_bound: Option<u64>) -> impl FnMut(&str) -> IResult<&str, MonkeyCabal> {
    move |input| {
        let mut i = 0;
        map(
            separated_list0(
                count(line_ending, 2),
                map_opt(monkey, move |(j, monkey)| {
                    let result = (i == j).then_some(monkey);
                    i += 1;
                    result
                }),
            ),
            |monkeys| (size_bound, monkeys).into(),
        )(input)
    }
}

pub fn level1(input: &str) -> usize {
    let mut monkeys = parse_input(Some(3))(input).unwrap().1;
    for _ in 0..20 {
        monkeys.round();
    }
    monkeys.monkey_business()
}

pub fn level2(input: &str) -> usize {
    let mut monkeys = parse_input(None)(input).unwrap().1;
    for _ in 0..10_000 {
        monkeys.round();
    }
    monkeys.monkey_business()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn level1_given_example() {
        let test_input = include_str!("./test_input/day11.txt");
        assert_eq!(level1(test_input), 10605)
    }

    #[test]
    fn level2_given_example() {
        let test_input = include_str!("./test_input/day11.txt");
        assert_eq!(level2(test_input), 2713310158)
    }
}
