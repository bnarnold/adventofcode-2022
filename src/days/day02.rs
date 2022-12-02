use itertools::Itertools;

use crate::util::*;

fn parse_line_score_level_1(line: &str) -> i64 {
    let shapes = line
        .split(' ')
        .map(|w| w.bytes().next().unwrap() as i64)
        .collect_vec();
    let x_ascii: i64 = "X".bytes().next().unwrap().into();
    let a_ascii: i64 = "A".bytes().next().unwrap().into();
    let shape_score = 1 + shapes[1] - x_ascii;
    let win_score = 3 * (shapes[1] - shapes[0] - 2 - x_ascii + a_ascii).rem_euclid(3);
    shape_score + win_score
}

fn parse_line_score_level_2(line: &str) -> i64 {
    let shapes = line
        .split(' ')
        .map(|w| w.bytes().next().unwrap() as i64)
        .collect_vec();
    let x_ascii: i64 = "X".bytes().next().unwrap().into();
    let a_ascii: i64 = "A".bytes().next().unwrap().into();

    let shape_score = 1 + (shapes[0] + shapes[1] - x_ascii - a_ascii - 1).rem_euclid(3);
    let win_score = 3 * (shapes[1] - x_ascii);
    shape_score + win_score
}

pub fn level1(input: &str) -> i64 {
    input.lines().map(parse_line_score_level_1).sum()
}

pub fn level2(input: &str) -> i64 {
    input.lines().map(parse_line_score_level_2).sum()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn level1_given_example() {
        let test_input = include_str!("./test_input/day02.txt");
        assert_eq!(level1(test_input), 15)
    }

    #[test]
    fn level2_given_example() {
        let test_input = include_str!("./test_input/day02.txt");
        assert_eq!(level2(test_input), 12)
    }
}
