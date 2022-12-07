use std::str::Lines;

use crate::util::prelude::*;

struct Sizes<'a> {
    stack: Vec<i64>,
    lines: Lines<'a>,
}

impl<'a> Iterator for Sizes<'a> {
    type Item = i64;

    fn next(&mut self) -> Option<Self::Item> {
        for line in self.lines.by_ref() {
            if line == "$ cd .." {
                if let Some(size) = self.stack.pop() {
                    if let Some(acc) = self.stack.last_mut() {
                        *acc += size;
                        return Some(size);
                    } else {
                        return None;
                    }
                }
            }
            if line.starts_with("$ cd ") {
                self.stack.push(0);
                continue;
            }
            if let Some(size) = line
                .split_once(' ')
                .and_then(|(s, _)| s.parse::<i64>().ok())
            {
                if let Some(acc) = self.stack.last_mut() {
                    *acc += size;
                }
            }
        }
        if let Some(size) = self.stack.pop() {
            if let Some(acc) = self.stack.last_mut() {
                *acc += size;
            }
            return Some(size);
        }
        None
    }
}

trait Sizeable {
    fn sizes(&self) -> Sizes;
}

impl Sizeable for str {
    fn sizes(&self) -> Sizes {
        Sizes {
            stack: Vec::new(),
            lines: self.lines(),
        }
    }
}

pub fn level1(input: &str) -> i64 {
    input.sizes().filter(|x| *x <= 100_000).sum()
}

pub fn level2(input: &str) -> i64 {
    let sizes = input.sizes().collect_vec();
    let cutoff = sizes.last().unwrap() - 40_000_000;
    sizes
        .into_iter()
        .filter(|size| *size >= cutoff)
        .min()
        .unwrap()
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn level1_given_example() {
        let test_input = include_str!("./test_input/day7.txt");
        assert_eq!(level1(test_input), 95437)
    }

    #[test]
    fn level2_given_example() {
        let test_input = include_str!("./test_input/day7.txt");
        assert_eq!(level2(test_input), 24933642)
    }
}
