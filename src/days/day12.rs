use std::{cell::Cell, collections::BinaryHeap, ops::ControlFlow};

use nom::{
    bytes::complete::{take_until, take_while},
    character::complete::{anychar, line_ending, satisfy},
    combinator::map_res,
    multi::many0,
    Parser,
};

use crate::util::prelude::*;

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
