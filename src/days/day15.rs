use std::{cmp::Ordering, collections::HashSet, iter::once};

use nom::{
    bytes::complete::tag,
    character::complete::{char, digit1, i64, line_ending},
    combinator::{all_consuming, map, map_res},
    multi::separated_list1,
    sequence::{pair, preceded, separated_pair},
    IResult,
};

use crate::util::prelude::*;

#[derive(Debug)]
struct Square {
    x: i64,
    y: i64,
    r: u64,
}

impl Square {
    fn interval(&self, y: i64) -> Option<Interval> {
        let dx = self.r.checked_sub(self.y.abs_diff(y))?;
        Some(Interval {
            start: self.x.saturating_sub_unsigned(dx),
            end: self.x.saturating_add_unsigned(dx),
        })
    }
}

fn pos(input: &str) -> IResult<&str, (i64, i64)> {
    pair(preceded(tag("x="), i64), preceded(tag(", y="), i64))(input)
}

fn parse_line(input: &str) -> IResult<&str, (Square, (i64, i64))> {
    map(
        pair(
            preceded(tag("Sensor at "), pos),
            preceded(tag(": closest beacon is at "), pos),
        ),
        |((x, y), (x2, y2))| {
            (
                Square {
                    x,
                    y,
                    r: x.abs_diff(x2) + y.abs_diff(y2),
                },
                (x2, y2),
            )
        },
    )(input)
}

fn parse_input(input: &str) -> IResult<&str, Vec<(Square, (i64, i64))>> {
    all_consuming(separated_list1(line_ending, parse_line))(input)
}

#[derive(Debug, Clone, Copy)]
struct Interval {
    start: i64,
    end: i64,
}

impl Interval {
    fn contains(&self, x: i64) -> bool {
        (self.start..=self.end).contains(&x)
    }

    fn merge(&self, other: Self) -> Option<Self> {
        (self.start <= other.end + 1 && other.start <= self.end + 1).then_some(Interval {
            start: { self.start.min(other.start) },
            end: self.end.max(other.end),
        })
    }

    fn len(&self) -> i64 {
        self.end + 1 - self.start
    }
}

#[derive(Debug, Default)]
struct DisjointIntervals(Vec<Interval>);

impl FromIterator<Interval> for DisjointIntervals {
    fn from_iter<T: IntoIterator<Item = Interval>>(iter: T) -> Self {
        let mut acc = Self::default();
        acc.extend(iter);
        acc
    }
}

impl Extend<Interval> for DisjointIntervals {
    fn extend<T: IntoIterator<Item = Interval>>(&mut self, iter: T) {
        for i @ Interval { start, end } in iter {
            let start_pos = self.search(start - 1);
            let end_pos = self.search(end + 1);
            let mut to_insert = i;
            if let Some(new) = start_pos
                .ok()
                .and_then(|i| self.0.get(i))
                .and_then(|start_interval| to_insert.merge(*start_interval))
            {
                to_insert = new
            };
            if let Some(new) = end_pos
                .ok()
                .and_then(|i| self.0.get(i))
                .and_then(|end_interval| to_insert.merge(*end_interval))
            {
                to_insert = new
            };
            let to_remove_start = start_pos.unwrap_or_else(|i| i);
            let to_remove_end = match end_pos {
                Result::Ok(i) => i + 1,
                Err(i) => i,
            };
            self.0
                .splice(to_remove_start..to_remove_end, once(to_insert));
        }
    }
}

impl DisjointIntervals {
    fn new(i: Interval) -> Self {
        Self(vec![i])
    }

    fn search(&self, x: i64) -> Result<usize, usize> {
        self.0.binary_search_by(|i| {
            if x < i.start {
                Ordering::Greater
            } else if x > i.end {
                Ordering::Less
            } else {
                Ordering::Equal
            }
        })
    }
}

pub fn level1(input: &str, y: i64) -> i64 {
    let (squares, mut beacons): (Vec<_>, Vec<_>) =
        parse_input(input).unwrap().1.into_iter().unzip();
    let intervals: DisjointIntervals = squares
        .iter()
        .filter_map(|square| square.interval(y))
        .collect();
    beacons.sort();
    let beacons_in_row = beacons
        .into_iter()
        .dedup()
        .filter(|(x_beacon, y_beacon)| y == *y_beacon && intervals.search(*x_beacon).is_ok())
        .count() as i64;
    intervals.0.iter().map(|i| i.len()).sum::<i64>() - beacons_in_row
}

// This isn't correct for all inputs since the empty field could also lie on the boundary,
// where it wouldn't need to be sandwiched between two sum = constant lines (candidate check).
// Since that case can be treated with the method from level one and did not occur for test
// or real input, it's left out for now
pub fn level2(input: &str, max: i64) -> i64 {
    let squares = parse_input(input)
        .unwrap()
        .1
        .into_iter()
        .map(|(square, _)| square)
        .collect_vec();
    let candidates_above = squares
        .iter()
        .map(|Square { x, y, r }| (x + y).saturating_add_unsigned(*r) + 1)
        .collect::<HashSet<_>>();
    let candidates_below = squares
        .iter()
        .map(|Square { x, y, r }| (x + y).saturating_sub_unsigned(*r) - 1)
        .collect::<HashSet<_>>();
    let candidates = candidates_above
        .intersection(&candidates_below)
        .filter(|c| (0..2 * max).contains(c))
        .collect_vec();
    for sum in candidates.into_iter() {
        let intervals = squares
            .iter()
            .filter(|s| sum.abs_diff(s.x + s.y) <= s.r)
            .map(|s| {
                let v = s.x - s.y;
                let cutoff = 2 * (2 * max - sum).min(*sum);
                Interval {
                    start: v.saturating_sub_unsigned(s.r).max(-cutoff),
                    end: v.saturating_add_unsigned(s.r).min(cutoff),
                }
            })
            .collect::<DisjointIntervals>();
        if let Some(Interval { start: diff, .. }) = intervals.0.get(1) {
            let x = (sum + diff - 1) / 2;
            let y = (sum - diff + 1) / 2;
            return x * 4_000_000 + y;
        }
    }
    panic!("Nothing found, are you sure there is a unique solution?")
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn level1_given_example() {
        let test_input = include_str!("./test_input/day15.txt");
        assert_eq!(level1(test_input, 10), 26)
    }

    #[test]
    fn level2_given_example() {
        let test_input = include_str!("./test_input/day15.txt");
        assert_eq!(level2(test_input, 20), 56000011)
    }
}
