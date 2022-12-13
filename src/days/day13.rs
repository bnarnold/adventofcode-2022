use std::cmp::Ordering;
use std::ops::ControlFlow;

use nom::branch::alt;
use nom::character::complete::{char, i32, line_ending};
use nom::combinator::map;
use nom::multi::{count, fold_many0, many_m_n, separated_list0};
use nom::sequence::{delimited, separated_pair};
use nom::IResult;

use crate::util::prelude::*;

#[derive(Debug, PartialEq, Eq)]
enum PacketData {
    Data(i32),
    List(Vec<PacketData>),
}

impl PacketData {
    fn divider(i: i32) -> Self {
        PacketData::List(vec![PacketData::List(vec![PacketData::Data(i)])])
    }
}

impl PartialOrd for PacketData {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for PacketData {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (PacketData::Data(x), PacketData::Data(y)) => x.cmp(y),
            (PacketData::Data(x), PacketData::List(ys)) => match ys.get(0) {
                Some(y) => PacketData::Data(*x).cmp(y).then(if ys.len() > 1 {
                    Ordering::Less
                } else {
                    Ordering::Equal
                }),
                None => Ordering::Greater,
            },
            (PacketData::List(xs), PacketData::Data(y)) => match xs.get(0) {
                Some(x) => PacketData::Data(*y).cmp(x).reverse().then(if xs.len() > 1 {
                    Ordering::Greater
                } else {
                    Ordering::Equal
                }),
                None => Ordering::Less,
            },
            (PacketData::List(xs), PacketData::List(ys)) => {
                if let ControlFlow::Break(result) =
                    xs.iter()
                        .zip_longest(ys.iter())
                        .try_for_each(|xy| match xy {
                            itertools::EitherOrBoth::Both(x, y) => match x.cmp(y) {
                                Ordering::Equal => ControlFlow::Continue(()),
                                result => ControlFlow::Break(result),
                            },
                            itertools::EitherOrBoth::Left(_) => {
                                ControlFlow::Break(Ordering::Greater)
                            }
                            itertools::EitherOrBoth::Right(_) => ControlFlow::Break(Ordering::Less),
                        })
                {
                    result
                } else {
                    Ordering::Equal
                }
            }
        }
    }
}

fn packet_data_list(input: &str) -> IResult<&str, Vec<PacketData>> {
    delimited(
        char('['),
        separated_list0(
            char(','),
            alt((
                map(packet_data_list, PacketData::List),
                map(i32, PacketData::Data),
            )),
        ),
        char(']'),
    )(input)
}

fn parse_line(input: &str) -> IResult<&str, PacketData> {
    map(packet_data_list, PacketData::List)(input)
}

fn parse_pair(input: &str) -> IResult<&str, Ordering> {
    map(
        separated_pair(parse_line, line_ending, parse_line),
        |(x, y)| x.cmp(&y),
    )(input)
}

fn parse_all_lines(input: &str) -> IResult<&str, Vec<PacketData>> {
    separated_list0(many_m_n(1, 2, line_ending), parse_line)(input)
}

pub fn level1(input: &str) -> usize {
    separated_list0(count(line_ending, 2), parse_pair)(input)
        .unwrap()
        .1
        .into_iter()
        .enumerate()
        .filter_map(|(i, c)| (c != Ordering::Greater).then_some(i + 1))
        .sum()
}

pub fn level2(input: &str) -> usize {
    let divider_two = PacketData::divider(2);
    let divider_six = PacketData::divider(6);
    let mut packets = parse_all_lines(input).unwrap().1;
    packets.push(PacketData::divider(2));
    packets.push(PacketData::divider(6));
    packets.sort();
    let two_pos = packets.iter().position(|p| p == &divider_two).unwrap() + 1;
    let six_pos = packets.iter().position(|p| p == &divider_six).unwrap() + 1;
    two_pos * six_pos
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn level1_given_example() {
        let test_input = include_str!("./test_input/day13.txt");
        assert_eq!(level1(test_input), 13)
    }

    #[test]
    fn level2_given_example() {
        let test_input = include_str!("./test_input/day13.txt");
        assert_eq!(level2(test_input), 140)
    }
}
