use anyhow::Context;
use aoc::{days::day12, util::infra::*};

fn main() {
    let (level, should_submit) = parse_args().unwrap();
    let input = include_str!("../../input/day12.txt");
    let data = match level {
        Level::One => day12::level1(input),
        Level::Two => day12::level2(input),
    };
    println!("{data}");
    if should_submit.is_some() {
        let day = 1;
        let session = std::env::var("SESSION")
            .context("SESSION must be set to submit")
            .unwrap();
        let _ = submit(day, level, data, session);
    }
}
