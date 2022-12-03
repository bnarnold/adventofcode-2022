pub use anyhow::*;
pub use itertools::Itertools;

pub fn ascii_code(c: char) -> i64 {
    c.to_string().bytes().next().unwrap() as i64
}

pub const LOWER_A_ASCII: i64 = 97;
pub const UPPER_A_ASCII: i64 = 65;
