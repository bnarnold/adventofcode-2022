use std::{cell::Cell, fmt::Display, iter::once, ops::ControlFlow};

use nom::{
    bytes::complete::{tag, take_while},
    character::complete::{char, digit1, line_ending},
    combinator::{all_consuming, map, map_res},
    multi::separated_list0,
    sequence::separated_pair,
    Finish, IResult,
};

use crate::util::prelude::*;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
struct GridPos {
    x: usize,
    y: usize,
}

impl GridPos {
    fn dist(&self, other: &Self) -> usize {
        ((self.x as isize - other.x as isize).abs() + (self.y as isize - other.y as isize).abs())
            as usize
    }

    fn neighbors(
        &self,
        min_x: usize,
        max_x: usize,
        max_y: usize,
    ) -> impl Iterator<Item = Option<GridPos>> + '_ {
        let x = self.x as isize;
        let y = self.y as isize;
        [(0, 1), (-1, 1), (1, 1)].into_iter().map(move |(dx, dy)| {
            let x2 = x + dx;
            let y2 = y + dy;
            ((((min_x as isize)..(max_x as isize)).contains(&x2))
                && (0..(max_y as isize)).contains(&y2))
            .then_some(Self {
                x: x2 as usize,
                y: y2 as usize,
            })
        })
    }
}

impl From<(usize, usize)> for GridPos {
    fn from(value: (usize, usize)) -> Self {
        Self {
            x: value.0,
            y: value.1,
        }
    }
}

fn usize(input: &str) -> IResult<&str, usize> {
    map_res(digit1, |s: &str| s.parse())(input)
}

fn grid_pos(input: &str) -> IResult<&str, GridPos> {
    map(separated_pair(usize, char(','), usize), |(x, y)| GridPos {
        x,
        y,
    })(input)
}

#[derive(Debug)]
struct GridBound {
    left: usize,
    right: usize,
    top: usize,
}

impl GridBound {
    fn from_pos(pos: &GridPos) -> Self {
        Self {
            left: pos.x,
            right: pos.x,
            top: pos.y,
        }
    }

    fn update_pos(&mut self, pos: &GridPos) {
        self.left = self.left.min(pos.x);
        self.right = self.right.max(pos.x);
        self.top = self.top.max(pos.y);
    }
}

#[derive(Debug, Copy, Clone)]
enum Location {
    Rock,
    Sand,
    Air,
}

impl Display for Location {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Location::Rock => '█',
            Location::Sand => '⣿',
            Location::Air => ' ',
        }
        .fmt(f)
    }
}

impl Location {
    fn is_free(&self) -> bool {
        matches!(*self, Location::Air)
    }
}

#[derive(Debug)]
struct Path(Vec<GridPos>);

fn path(input: &str) -> IResult<&str, Path> {
    map(separated_list0(tag(" -> "), grid_pos), Path)(input)
}

#[derive(Debug)]
struct Grid {
    inner: Vec<Cell<Location>>,
    x_offset: usize,
    length: usize,
    height: usize,
}

impl Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for line in self.inner.chunks(self.length) {
            for pos in line.iter() {
                pos.get().fmt(f)?
            }
            writeln!(f)?
        }
        std::fmt::Result::Ok(())
    }
}

impl Grid {
    fn contains(&self, pos: &GridPos) -> bool {
        (self.x_offset..(self.x_offset + self.length)).contains(&pos.x)
            && (0..self.height).contains(&pos.y)
    }
    fn get(&self, pos: &GridPos) -> Option<Location> {
        if self.contains(pos) {
            self.inner
                .get(pos.x - self.x_offset + self.length * pos.y)
                .map(Cell::get)
        } else {
            None
        }
    }

    fn set(&self, pos: &GridPos, loc: Location) -> bool {
        if self.contains(pos) {
            self.inner
                .get(pos.x - self.x_offset + self.length * pos.y)
                .map(|cell| {
                    cell.set(loc);
                    true
                })
                .unwrap_or(false)
        } else {
            false
        }
    }

    fn neighbors<'a, 'b: 'a>(
        &'a self,
        pos: &'b GridPos,
    ) -> impl Iterator<Item = Option<(GridPos, Location)>> + 'a {
        self.contains(pos).then_some(()).into_iter().flat_map(|_| {
            pos.neighbors(self.x_offset, self.x_offset + self.length, self.height)
                .map(|new_pos| {
                    let new_pos = new_pos?;
                    let loc = self.get(&new_pos)?;
                    Some((new_pos, loc))
                })
        })
    }

    fn new(paths: Vec<Path>) -> Self {
        let Some(GridBound { left, right, top }) = 
            paths.iter()
                 .flat_map(|path|path.0.iter())
                 .fold(None::<GridBound>,|acc,pos| match acc {
                    Some(mut bounds) => {bounds.update_pos(pos); Some(bounds)},
                    None => Some(GridBound::from_pos(pos)),
                 })
        else {return Self { inner: Vec::new(), x_offset: 0, length: 0, height: 0 }};
        let length = right - left + 1;
        let height = top + 1;
        let mut result = Self {
            inner: vec![Cell::new(Location::Air); length * height],
            x_offset: left,
            length,
            height,
        };
        paths.into_iter().for_each(|path| result.add_path(path));
        result
    }

    fn add_path(&mut self, Path(nodes): Path) {
        let mut nodes = nodes.iter();
        let Some(mut start_pos) = nodes.next() else {return};
        for end_pos in nodes {
            if start_pos.x == end_pos.x {
                let start_y = start_pos.y.min(end_pos.y);
                let end_y = start_pos.y.max(end_pos.y);
                for y in start_y..=end_y {
                    self.set(&GridPos { x: start_pos.x, y }, Location::Rock);
                }
            } else if start_pos.y == end_pos.y {
                let start_x = start_pos.x.min(end_pos.x);
                let end_x = start_pos.x.max(end_pos.x);
                for x in start_x..=end_x {
                    self.set(&GridPos { x, y: start_pos.y }, Location::Rock);
                }
            }
            start_pos = end_pos;
        }
    }

    fn drop_sand(&self, start_pos: GridPos) -> bool {
        let mut pos = start_pos;
        'fall: loop {
            for next in self.neighbors(&pos.clone()) {
                match next {
                    None => return false, // falling outside the grid
                    Some((next_pos, Location::Air)) => {
                        pos = next_pos; //falling to a free spot in the grid
                        continue 'fall;
                    }
                    _ => {}
                }
            }
            return self.set(&pos, Location::Sand); // no free spots below, settling
        }
    }

    fn get_sandy_count(&self, start_x: usize) -> usize {
        // In the end, exactly those locations which can be reached from the
        // start position by going down one and at most one to a side
        // will be sandy. On the left and right of the grid, this gives two
        // triangles whose area can be calculated from the height.
        // This leaves the interior, which can be calculated by scanning
        // through the rows in O(length * height).
        let mut left_escape: Option<usize> = None;
        let mut right_escape: Option<usize> = None;
        let mut sandy = vec![false; self.length];
        let mut sandy_count = 1;
        sandy[start_x - self.x_offset] = true;
        let last_row: &[Cell<Location>] = &vec![Cell::new(Location::Air); self.length];
        eprintln!();
        for (i, is_sandy) in sandy.iter().enumerate() {
            eprint!(
                "{}",
                if *is_sandy {
                    Location::Sand
                } else {
                    self.inner[i].get()
                }
            );
        }
        for (i, row) in self
            .inner
            .chunks(self.length)
            .chain(once(last_row))
            .enumerate()
            .skip(1)
        {
            let mut new_sandy = vec![false; self.length];
            for (i, is_sandy) in new_sandy.iter_mut().enumerate() {
                *is_sandy = sandy[i.saturating_sub(1)..=(i + 1).min(self.length - 1)]
                    .iter()
                    .any(|is_sandy| *is_sandy);
            }
            if left_escape.is_some() {
                new_sandy[0] = true;
            }
            if right_escape.is_some() {
                *new_sandy.last_mut().unwrap() = true;
            }
            for (is_sandy, loc) in new_sandy.iter_mut().zip(row.iter()) {
                *is_sandy = *is_sandy && !matches!(loc.get(), Location::Rock);
                if *is_sandy {
                    sandy_count += 1
                }
            }
            if left_escape.is_none() && sandy[0] {
                left_escape.replace(i);
            }
            if right_escape.is_none() && *sandy.last().unwrap() {
                right_escape.replace(i);
            }
            sandy = new_sandy;
            eprintln!();
            for (i, is_sandy) in sandy.iter().enumerate() {
                eprint!(
                    "{}",
                    if *is_sandy {
                        Location::Sand
                    } else {
                        row[i].get()
                    }
                );
            }
        }
        eprintln!();
        let left_height = left_escape.map(|h| self.height + 1 - h).unwrap_or_default();
        let right_height = right_escape
            .map(|h| self.height + 1 - h)
            .unwrap_or_default();
        sandy_count + (left_height * (left_height + 1) + right_height * (right_height + 1)) / 2
    }
}

pub fn level1(input: &str) -> usize {
    let paths = all_consuming(separated_list0(line_ending, path))(input)
        .finish()
        .unwrap()
        .1;
    let grid = Grid::new(paths);
    let mut count = 0;
    while grid.drop_sand(GridPos { x: 500, y: 0 }) {
        count += 1;
    }
    count
}

pub fn level2(input: &str) -> usize {
    let paths = all_consuming(separated_list0(line_ending, path))(input)
        .finish()
        .unwrap()
        .1;
    Grid::new(paths).get_sandy_count(500)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn level1_given_example() {
        let test_input = include_str!("./test_input/day14.txt");
        assert_eq!(level1(test_input), 24)
    }

    #[test]
    fn level2_given_example() {
        let test_input = include_str!("./test_input/day14.txt");
        assert_eq!(level2(test_input), 93)
    }
}
