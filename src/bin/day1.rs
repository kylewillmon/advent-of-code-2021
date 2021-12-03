use std::collections::VecDeque;

use aoc::{aoc_main, Result};

use nom::character::complete::{line_ending, u64};
use nom::combinator::eof;
use nom::multi::many0;
use nom::multi::separated_list1;
use nom::sequence::preceded;
use nom::sequence::terminated;
use nom::IResult;

fn main() {
    aoc_main(1, parse_nums, part_one, part_two).unwrap()
}

fn part_one(nums: Vec<u64>) -> Result<u64> {
    let mut latest = nums.get(0).cloned().unwrap_or(0u64);
    let mut count = 0;

    for i in nums.iter().cloned().skip(1) {
        if i > latest {
            count += 1;
        }
        latest = i;
    }

    Ok(count)
}

fn part_two(nums: Vec<u64>) -> Result<u64> {
    let mut latest: VecDeque<u64> = nums.get(..=2).unwrap_or(&[]).iter().cloned().collect();
    let mut count = 0;

    for i in nums.iter().cloned().skip(3) {
        if i > latest.pop_front().unwrap() {
            count += 1;
        }
        latest.push_back(i);
    }
    Ok(count)
}

fn parse_nums(s: &str) -> Result<Vec<u64>> {
    fn inner(s: &str) -> IResult<&str, Vec<u64>> {
        terminated(
            separated_list1(line_ending, u64),
            preceded(many0(line_ending), eof),
        )(s)
    }
    match inner(s) {
        Ok((_s, v)) => Ok(v),
        Err(e) => Err(e.to_owned().into()),
    }
}
