use itertools::Itertools;

use crate::util::*;

pub fn level1(input: &str) -> i64 {
    let groups = input
        .split("\n\n")
        .map(|group| group.lines().map(|x| x.parse::<i64>().unwrap()).sum());
    groups.max().unwrap()
}

pub fn level2(input: &str) -> i64 {
    let mut groups = input
        .split("\n\n")
        .map(|group| group.lines().map(|x| x.parse::<i64>().unwrap()).sum())
        .collect_vec();
    groups.sort();
    groups[groups.len() - 3..].iter().sum()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn level1_given_example() {
        let test_input = include_str!("./test_input/day1.txt");
        assert_eq!(level1(test_input), 24000)
    }

    #[test]
    fn level2_given_example() {
        let test_input = include_str!("./test_input/day1.txt");
        assert_eq!(level2(test_input), 45000)
    }
}
