use std::{fmt::Display, str::FromStr};

use anyhow::{anyhow, Context};

#[derive(Debug)]
pub enum Level {
    One,
    Two,
}

impl FromStr for Level {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "1" => Ok(Level::One),
            "2" => Ok(Level::Two),
            _ => Err(anyhow!("Expected one of 1, 2")),
        }
    }
}

impl Display for Level {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        {
            match self {
                Level::One => "1",
                Level::Two => "2",
            }
        }
        .fmt(f)
    }
}

#[derive(Debug)]
pub struct Submit;

pub fn parse_args() -> anyhow::Result<(Level, Option<Submit>)> {
    let mut pargs = pico_args::Arguments::from_env();
    let args = (
        pargs
            .value_from_str("--level")
            .context("must pass --level")?,
        pargs.contains(["-s", "--submit"]).then_some(Submit),
    );
    Ok(args)
}

pub fn submit(
    day: u32,
    level: Level,
    data: impl Display,
    session: String,
) -> anyhow::Result<ureq::Response> {
    let url = format!("https://adventofcode.com/2022/day/{day}/answer");
    let session_cookie = format!("session={session}");
    let payload = format!("level={level}&answer={data}");
    ureq::post(&url)
        .set("Content-Type", "x-www-form-urlencoded")
        .set("Cookie", &session_cookie)
        .send_string(&payload)
        .context("Submit failed")
}
