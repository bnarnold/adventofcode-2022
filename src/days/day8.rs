use std::cmp::Ordering;

use crate::util::prelude::*;

fn parse_with_default<T: Clone>(input: &str, default: &T) -> Vec<Vec<(u32, T)>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .filter_map(|c| c.to_digit(10).map(|x| (x, default.clone())))
                .collect_vec()
        })
        .collect()
}

fn transpose<T>(table: Vec<Vec<T>>) -> Vec<Vec<T>> {
    let row_len = if let Some(row) = table.get(0) {
        row.len()
    } else {
        return Vec::new();
    };
    let col_len = table.len();
    let mut acc = Vec::with_capacity(row_len);
    for _ in 0..row_len {
        acc.push(Vec::with_capacity(col_len));
    }
    table.into_iter().for_each(|row| {
        acc.iter_mut()
            .zip(row)
            .for_each(|(col, entry)| col.push(entry))
    });
    acc
}

fn set_visible<'a, T, I>(row: I)
where
    T: Ord + 'a,
    I: Iterator<Item = &'a mut (T, bool)>,
{
    let mut acc: Option<&T> = None;
    for (t, visible) in row {
        if let Some(max_so_far) = acc {
            if *t > *max_so_far {
                *visible = true;
                acc = Some(t)
            }
        } else {
            *visible = true;
            acc = Some(t)
        }
    }
}

fn set_row_visible(table: &mut [Vec<(impl Ord, bool)>]) {
    table.iter_mut().for_each(|row| {
        set_visible(row.iter_mut());
        set_visible(row.iter_mut().rev())
    })
}

fn set_visible_count<'a, T, I>(row: I)
where
    T: Ord + 'a,
    I: Iterator<Item = &'a mut (T, usize)>,
{
    // List of (height, visible trees including self) for potentially visible trees
    let mut visible_trees: Vec<(&'a T, usize)> = Vec::new();
    for (t, visible_count) in row {
        let split_point = visible_trees
            .binary_search_by(|(x, _)| {
                if **x >= *t {
                    Ordering::Less // Larger or equal are before split
                } else {
                    Ordering::Greater // Smaller are after
                }
            })
            .unwrap_err(); // Can't panic since comparing never gives Equalcode
        let visible: usize = visible_trees
            .split_off(split_point)
            .into_iter()
            .map(|(_, visible_count)| visible_count)
            .sum();
        *visible_count *= visible + if visible_trees.is_empty() { 0 } else { 1 };
        visible_trees.push((t, visible + 1))
    }
}

fn set_row_visible_count(table: &mut [Vec<(impl Ord, usize)>]) {
    table.iter_mut().for_each(|row| {
        set_visible_count(row.iter_mut());
        set_visible_count(row.iter_mut().rev())
    })
}

pub fn level1(input: &str) -> usize {
    let mut table = parse_with_default(input, &false);
    set_row_visible(&mut table);
    table = transpose(table);
    set_row_visible(&mut table);
    table
        .into_iter()
        .flat_map(|row| row.into_iter().filter(|(_, visible)| *visible))
        .count()
}

pub fn level2(input: &str) -> usize {
    let mut table = parse_with_default(input, &1_usize);
    set_row_visible_count(&mut table);
    table = transpose(table);
    set_row_visible_count(&mut table);
    table
        .into_iter()
        .flat_map(|row| row.into_iter().map(|(_, visible_count)| visible_count))
        .max()
        .unwrap()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn level1_given_example() {
        let test_input = include_str!("./test_input/day8.txt");
        assert_eq!(level1(test_input), 21)
    }

    #[test]
    fn level2_given_example() {
        let test_input = include_str!("./test_input/day8.txt");
        assert_eq!(level2(test_input), 8)
    }
}
