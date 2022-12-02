use anyhow::Context;
use aoc::{days::day02, util::infra::*};

fn main() {
    let (level, should_submit) = parse_args().unwrap();
    let input = include_str!("../../input/day02.txt");
    let data = match level {
        Level::One => day02::level1(input),
        Level::Two => day02::level2(input),
    };
    println!("{data}");
    if should_submit.is_some() {
        let day = 1;
        let session = std::env::var("SESSION")
            .context("SESSION must be set to submit")
            .unwrap();
        let resp = submit(day, level, data, session);
    }
}
