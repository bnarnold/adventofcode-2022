use std::{cell::Cell, collections::BinaryHeap, ops::ControlFlow};

use nom::{
    bytes::complete::{take_until, take_while},
    character::complete::{anychar, line_ending, satisfy},
    combinator::map_res,
    multi::many0,
    IResult, Parser,
};

use crate::util::prelude::*;

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

#[derive(Debug)]
enum Tree {
    Tree(i64),
    Start,
    End,
}

impl Tree {
    fn height(&self) -> i64 {
        match *self {
            Tree::Tree(height) => height,
            Tree::Start => 0,
            Tree::End => 25,
        }
    }
}

impl TryFrom<char> for Tree {
    type Error = anyhow::Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'a'..='z' => Ok(Tree::Tree(ascii_code(value) - LOWER_A_ASCII)),
            'S' => Ok(Tree::Start),
            'E' => Ok(Tree::End),
            _ => Err(anyhow!("Expected S, E, or lowercase ASCII character")),
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
struct SearchEntry {
    priority: usize,
    depth: usize,
    position: GridPos,
}

impl From<(usize, usize, GridPos)> for SearchEntry {
    fn from((priority, depth, position): (usize, usize, GridPos)) -> Self {
        Self {
            priority,
            depth,
            position,
        }
    }
}

impl PartialOrd for SearchEntry {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for SearchEntry {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.priority
            .cmp(&other.priority)
            .then(self.depth.cmp(&other.depth))
            .reverse()
    }
}

fn parse_grid(input: &str) -> Option<(Grid<(Tree, Cell<bool>)>, GridPos, GridPos)> {
    let mut y = 0;
    let mut start_pos: Option<GridPos> = None;
    let mut end_pos: Option<GridPos> = None;
    let (_, grid) = Grid::parse(|line| {
        map_res(take_while(|c: char| c.is_alphabetic()), |line: &str| {
            let result = line
                .chars()
                .enumerate()
                .map(|(x, c)| {
                    c.try_into().map(|t: Tree| {
                        match t {
                            Tree::Start => start_pos = Some(GridPos { x, y }),
                            Tree::End => end_pos = Some(GridPos { x, y }),
                            _ => {}
                        };
                        (t, false.into())
                    })
                })
                .collect();
            y += 1;
            result
        })(line)
    })
    .parse(input)
    .ok()?;
    Some((grid, start_pos?, end_pos?))
}

fn a_star(
    grid: Grid<(Tree, Cell<bool>)>,
    start_pos: GridPos,
    is_end: impl Fn(&Tree) -> bool,
    priority: impl Fn(usize, &GridPos) -> usize,
    cost: impl Fn(&Tree, &Tree) -> Option<usize>,
) -> Option<usize> {
    grid.get(&start_pos).map(|(_, visited)| visited.set(true));
    let mut queue: BinaryHeap<SearchEntry> = BinaryHeap::new();
    queue.push((priority(0, &start_pos), 0, start_pos).into());
    while let Some(SearchEntry {
        depth, position, ..
    }) = queue.pop()
    {
        if let ControlFlow::Break(result) = match grid.get(&position) {
            Some((tree, _)) => grid
                .neighbors(&position)
                .try_for_each(|(new_pos, t)| match t {
                    (new_tree, new_visited) => match cost(&tree, &new_tree) {
                        Some(move_cost) if !new_visited.get() => {
                            if is_end(new_tree) {
                                return ControlFlow::Break(depth + move_cost);
                            }
                            let priority = priority(depth + move_cost, &new_pos);
                            new_visited.set(true);
                            queue.push(SearchEntry {
                                priority,
                                depth: depth + move_cost,
                                position: new_pos,
                            });
                            ControlFlow::Continue(())
                        }
                        _ => ControlFlow::Continue(()),
                    },
                }),
            _ => ControlFlow::Continue(()),
        } {
            return Some(result);
        }
    }
    None
}

pub fn level1(input: &str) -> usize {
    let (grid, start_pos, end_pos) = parse_grid(input).unwrap();
    a_star(
        grid,
        start_pos,
        |tree| matches!(tree, Tree::End),
        |depth, pos| depth + pos.dist(&end_pos),
        |start, end| (end.height() <= start.height() + 1).then_some(1),
    )
    .unwrap()
}

pub fn level2(input: &str) -> usize {
    let (grid, _, end_pos) = parse_grid(input).unwrap();
    a_star(
        grid,
        end_pos,
        |tree| tree.height() == 0,
        |depth, _| depth,
        |start, end| (start.height() <= end.height() + 1).then_some(1),
    )
    .unwrap()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn level1_given_example() {
        let test_input = include_str!("./test_input/day12.txt");
        assert_eq!(level1(test_input), 31)
    }

    #[test]
    fn level2_given_example() {
        let test_input = include_str!("./test_input/day12.txt");
        assert_eq!(level2(test_input), 29)
    }
}
