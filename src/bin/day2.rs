use std::str::FromStr;

use aoc::{aoc_main, Result};

fn main() {
    aoc_main(2, parse_commands, part_one, part_two).unwrap()
}

fn part_one(commands: Vec<Command>) -> Result<i64> {
    let mut horizontal = 0;
    let mut depth = 0;

    for cmd in commands.iter().cloned() {
        match cmd {
            Command::Forward(amt) => {
                horizontal += amt;
            }
            Command::Up(amt) => {
                depth -= amt;
            }
            Command::Down(amt) => {
                depth += amt;
            }
        }
    }
    Ok(horizontal * depth)
}

fn part_two(commands: Vec<Command>) -> Result<i64> {
    let mut horizontal = 0;
    let mut depth = 0;
    let mut aim = 0;

    for cmd in commands.iter().cloned() {
        match cmd {
            Command::Forward(amt) => {
                horizontal += amt;
                depth += amt * aim;
            }
            Command::Up(amt) => {
                aim -= amt;
            }
            Command::Down(amt) => {
                aim += amt;
            }
        }
    }
    Ok(horizontal * depth)
}

#[derive(Debug, Clone)]
enum Command {
    Forward(i64),
    Up(i64),
    Down(i64),
}

impl FromStr for Command {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let (dir, amount) = s
            .split_once(' ')
            .ok_or(anyhow::anyhow!("invalid command"))?;

        let amount = i64::from_str(amount)?;

        let cmd = if dir.eq_ignore_ascii_case("forward") {
            Command::Forward(amount)
        } else if dir.eq_ignore_ascii_case("up") {
            Command::Up(amount)
        } else if dir.eq_ignore_ascii_case("down") {
            Command::Down(amount)
        } else {
            anyhow::bail!("invalid direction: {}", dir);
        };
        Ok(cmd)
    }
}

fn parse_commands(s: &str) -> Result<Vec<Command>> {
    let mut v = Vec::new();
    for l in s.lines() {
        v.push(Command::from_str(l)?);
    }
    Ok(v)
}
