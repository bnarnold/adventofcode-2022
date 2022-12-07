use std::{collections::HashMap, hash::Hash};

use nom::{
    branch::alt,
    bytes::complete::{tag, take_while, take_while1},
    character::complete::{alphanumeric1, char, i64, newline, satisfy},
    combinator::{eof, map, opt, value, verify},
    multi::{many0, many1, separated_list0, separated_list1},
    sequence::{delimited, pair, preceded, terminated, tuple},
    IResult,
};

#[derive(Debug)]
enum Entry<'a> {
    File {
        size: i64,
    },
    Dir {
        content: HashMap<&'a str, Entry<'a>>,
    },
}

impl<'a> Entry<'a> {
    pub fn size(&self) -> i64 {
        match self {
            Entry::File { size } => *size,
            Entry::Dir { content } => content.values().map(Self::size).sum(),
        }
    }

    fn all_sizes_with_own_size(&self) -> (Vec<i64>, i64) {
        match self {
            Entry::File { size } => (vec![], *size),
            Entry::Dir { content } => {
                let (mut children_sizes, entry_size) =
                    content.values().map(Self::all_sizes_with_own_size).fold(
                        (vec![], 0),
                        |(mut sizes, acc), (mut children_sizes, entry_size)| {
                            sizes.append(&mut children_sizes);
                            (sizes, acc + entry_size)
                        },
                    );
                children_sizes.push(entry_size);
                (children_sizes, entry_size)
            }
        }
    }

    pub fn all_sizes(&self) -> Vec<i64> {
        self.all_sizes_with_own_size().0
    }
}

use crate::util::prelude::*;

fn path_component(s: &str) -> IResult<&str, &str> {
    verify(take_while1(|c: char| !c.is_whitespace()), |x: &str| {
        x != ".."
    })(s)
}

fn cd_child_line(line: &str) -> IResult<&str, &str> {
    delimited(tag("$ cd "), path_component, newline)(line)
}

fn cd_parent_line(line: &str) -> IResult<&str, ()> {
    value((), pair(newline, tag("$ cd ..")))(line)
}

fn ls_command(lines: &str) -> IResult<&str, Vec<(&str, Entry)>> {
    map(
        preceded(
            pair(tag("$ ls"), newline),
            separated_list0(newline, ls_output_line),
        ),
        |maybe_entries| maybe_entries.into_iter().flatten().collect(),
    )(lines)
}

fn ls_output_line(line: &str) -> IResult<&str, Option<(&str, Entry)>> {
    alt((
        map(pair(tag("dir "), path_component), |_| None),
        map(
            pair(i64, preceded(char(' '), path_component)),
            |(size, name)| Some((name, Entry::File { size })),
        ),
    ))(line)
}

fn cd_command(line: &str) -> IResult<&str, (&str, Entry)> {
    terminated(
        pair(
            cd_child_line,
            map(
                separated_list0(
                    newline,
                    alt((map(cd_command, |data| vec![data]), ls_command)),
                ),
                |entries| Entry::Dir {
                    content: entries.into_iter().flatten().collect(),
                },
            ),
        ),
        alt((
            value((), cd_parent_line),
            value((), pair(opt(newline), eof)),
        )),
    )(line)
}

pub fn level1(input: &str) -> i64 {
    let (_, (_, entry)) = cd_command(input).unwrap();
    entry
        .all_sizes()
        .into_iter()
        .filter(|x| *x <= 100_000)
        .sum()
}

pub fn level2(input: &str) -> i64 {
    let (_, (_, entry)) = cd_command(input).unwrap();
    let (sizes, total_size) = entry.all_sizes_with_own_size();
    let cutoff = total_size - 40_000_000;
    sizes.into_iter().filter(|x| *x >= cutoff).min().unwrap()
}

#[cfg(test)]
mod test {
    use itertools::assert_equal;

    use super::*;

    #[test]
    fn ls_command_returns_array_of_entries() {
        let input = r#"$ ls
dir a
123 file1
456 file2
dir b"#;
        let (rest, entries) = ls_command(input).unwrap();
        assert_equal(
            entries.into_iter().map(|(s, e)| (s, e.size())),
            vec![("file1", 123), ("file2", 456)].into_iter(),
        );
        assert_eq!(rest, "");
    }

    #[test]
    fn cd_command_recurses() {
        let input = r#"$ cd a
$ cd b
$ cd ..
$ cd c
$ cd .."#;
        if let (rest, (name, Entry::Dir { content })) = cd_command(input).unwrap() {
            assert_eq!(name, "a");
            assert_eq!(
                content.keys().cloned().sorted().collect_vec(),
                vec!["b", "c"]
            );
            assert_eq!(rest, "")
        } else {
            panic!("Expected directory")
        };
    }

    #[test]
    fn can_mix_ls_and_cd() {
        let input = r#"$ cd x
$ ls
123 file1
$ cd a
$ ls
456 file2
$ cd ..
$ cd b
$ ls
789 file3"#;
        let (rest, (_, entry)) = cd_command(input).unwrap();
        assert_eq!(entry.size(), 1368);
        assert_eq!(rest, "");
    }

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
