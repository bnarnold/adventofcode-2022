pub use anyhow::*;
pub use itertools::Itertools;
use nom::{character::complete::line_ending, IResult, Parser};

pub fn ascii_code(c: char) -> i64 {
    c.to_string().bytes().next().unwrap() as i64
}

pub const LOWER_A_ASCII: i64 = 97;
pub const UPPER_A_ASCII: i64 = 65;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct GridPos {
    pub x: usize,
    pub y: usize,
}

impl GridPos {
    pub fn dist(&self, other: &Self) -> usize {
        ((self.x as isize - other.x as isize).abs() + (self.y as isize - other.y as isize).abs())
            as usize
    }

    pub fn neighbors(&self, max_x: usize, max_y: usize) -> impl Iterator<Item = GridPos> + '_ {
        let x = self.x as isize;
        let y = self.y as isize;
        [(-1, 0), (1, 0), (0, -1), (0, 1)]
            .into_iter()
            .map(move |(dx, dy)| (x + dx, y + dy))
            .filter_map(move |(x2, y2)| {
                (((0..(max_x as isize)).contains(&x2)) && (0..(max_y as isize)).contains(&y2))
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

#[derive(Debug)]
pub struct Grid<T> {
    inner: Vec<T>,
    pub length: usize,
    pub height: usize,
}

impl<T> Grid<T> {
    pub fn contains(&self, pos: &GridPos) -> bool {
        (0..self.length).contains(&pos.x) && (0..self.height).contains(&pos.y)
    }
    pub fn get(&self, pos: &GridPos) -> Option<&T> {
        if self.contains(pos) {
            self.inner.get(pos.x + self.length * pos.y)
        } else {
            None
        }
    }
    pub fn get_mut(&mut self, pos: &GridPos) -> Option<&mut T> {
        if self.contains(pos) {
            self.inner.get_mut(pos.x + self.length * pos.y)
        } else {
            None
        }
    }

    pub fn iter_rows(&self) -> impl Iterator<Item = std::slice::Iter<T>> {
        self.inner.chunks(self.length).map(|s| s.iter())
    }

    pub fn iter_rows_mut(&mut self) -> impl Iterator<Item = std::slice::IterMut<T>> {
        self.inner.chunks_mut(self.length).map(|s| s.iter_mut())
    }

    pub fn neighbors<'a, 'b: 'a>(
        &'a self,
        pos: &'b GridPos,
    ) -> impl Iterator<Item = (GridPos, &'a T)> + 'a {
        self.contains(pos).then_some(()).into_iter().flat_map(|_| {
            pos.neighbors(self.length, self.height)
                .filter_map(|new_pos| self.get(&new_pos).map(|t| (new_pos, t)))
        })
    }

    pub fn parse<'a, F: Parser<&'a str, Vec<T>, nom::error::Error<&'a str>>>(
        mut line_parser: F,
    ) -> impl Parser<&'a str, Self, nom::error::Error<&'a str>> {
        move |input| match line_parser.parse(input) {
            IResult::Ok((input, first_line)) => {
                let length = first_line.len();
                let mut acc = first_line;
                let mut height = 1;
                let mut rest = input;
                loop {
                    match line_ending::<_, nom::error::Error<&'a str>>(rest) {
                        IResult::Ok((new_rest, _)) => rest = new_rest,
                        Err(_) => {
                            return IResult::Ok((
                                input,
                                Self {
                                    inner: acc,
                                    length,
                                    height,
                                },
                            ))
                        }
                    }
                    match line_parser.parse(rest) {
                        IResult::Ok((new_rest, mut row)) if row.len() == length => {
                            acc.append(&mut row);
                            height += 1;
                            rest = new_rest;
                        }
                        _ => {
                            return IResult::Ok((
                                input,
                                Self {
                                    inner: acc,
                                    length,
                                    height,
                                },
                            ))
                        }
                    }
                }
            }
            Err(e) => Err(e),
        }
    }
}

impl<J, T> FromIterator<J> for Grid<T>
where
    J: Iterator<Item = T>,
{
    fn from_iter<I: IntoIterator<Item = J>>(iter: I) -> Self {
        let mut iter = iter.into_iter();
        let mut height = 0;
        if let Some(first) = iter.next() {
            let mut acc = first.collect_vec();
            let length = acc.len();
            for row in iter {
                acc.extend(row);
                if acc.len() - height * length != length {
                    panic!("Expected {length} elements")
                }
                height += 1;
            }
            Self {
                inner: acc,
                height,
                length,
            }
        } else {
            Self {
                inner: Vec::new(),
                length: 0,
                height: 0,
            }
        }
    }
}
