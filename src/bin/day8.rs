use std::str::FromStr;

use anyhow::{anyhow, bail};
use aoc::{aoc_main, Result};
use bitvec::prelude::*;

fn main() {
    aoc_main(
        8,
        |s| {
            let v: Result<Vec<_>, _> = s.lines().map(Line::from_str).collect();
            v
        },
        part_one,
        part_two,
    )
    .unwrap()
}

fn part_one(lines: Vec<Line>) -> Result<usize> {
    Ok(lines
        .into_iter()
        .map(|line| {
            line.output
                .iter()
                .filter(|num| matches!(num.count_segments(), 2 | 3 | 4 | 7))
                .count()
        })
        .sum())
}

fn part_two(lines: Vec<Line>) -> Result<usize> {
    lines.into_iter().map(|line| line.solve()).sum()
}

#[derive(Clone, Debug)]
struct Line {
    nums: [Number; 10],
    output: [Number; 4],
}

impl Line {
    fn mapping(&self) -> Result<[Segment; 7]> {
        let mut mapping = [Segment(0); 7];
        for i in 0..7 {
            let s = Segment(i);
            mapping[i as usize] = s
                .deduce(&self.nums)
                .ok_or_else(|| anyhow!("no match found!"))?;
        }
        Ok(mapping)
    }

    fn solve(self) -> Result<usize> {
        let mapping = self.mapping()?;

        let mut out = 0;
        for num in self.output.into_iter() {
            out *= 10;
            let d = num
                .with_mapping(mapping)
                .decode()
                .ok_or_else(|| anyhow!("output does not match any known number"))?;
            out += d;
        }

        Ok(out)
    }
}

impl FromStr for Line {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (nums, output) = s
            .trim()
            .split_once(" | ")
            .ok_or_else(|| anyhow!("no pipe found"))?;

        fn parse_nums(s: &str) -> Result<Vec<Number>> {
            s.split_whitespace().map(Number::from_str).collect()
        }
        Ok(Line {
            nums: parse_nums(nums)?
                .try_into()
                .map_err(|v: Vec<Number>| anyhow!("invalid length: {}", v.len()))?,
            output: parse_nums(output)?
                .try_into()
                .map_err(|v: Vec<Number>| anyhow!("invalid length: {}", v.len()))?,
        })
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Number(BitArr!(for 7, in Msb0, u8));

impl Number {
    fn count_segments(&self) -> usize {
        self.0.count_ones()
    }

    fn has_segment(&self, segment: Segment) -> bool {
        self.0
            .get(segment.0 as usize)
            .as_deref()
            .cloned()
            .unwrap_or(false)
    }

    fn from_segments<S: IntoIterator<Item = Segment>>(segments: S) -> Self {
        let mut bits = bitarr![Msb0, u8; 0; 7];
        for seg in segments.into_iter() {
            bits.get_mut(seg.0 as usize).unwrap().set(true);
        }
        Number(bits)
    }

    fn with_mapping(&self, mapping: [Segment; 7]) -> Self {
        let mut bits = bitarr![Msb0, u8; 0; 7];
        for i in self.0.iter_ones() {
            bits.get_mut(mapping[i].0 as usize).unwrap().set(true);
        }
        Number(bits)
    }

    fn decode(&self) -> Option<usize> {
        for (i, num) in real_nums().into_iter().enumerate() {
            if num == *self {
                return Some(i);
            }
        }
        None
    }
}

impl FromStr for Number {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let segments: Result<Vec<_>> = s.trim().chars().map(Segment::try_from).collect();
        Ok(Number::from_segments(segments?))
    }
}

macro_rules! number {
    ($($s:literal),+) => (
        Number::from_segments([
            $(Segment($s)),+
        ])
    );
}

fn real_nums() -> [Number; 10] {
    [
        number!(0, 1, 2, 4, 5, 6),    // 0
        number!(2, 5),                // 1
        number!(0, 2, 3, 4, 6),       // 2
        number!(0, 2, 3, 5, 6),       // 3
        number!(1, 2, 3, 5),          // 4
        number!(0, 1, 3, 5, 6),       // 5
        number!(0, 1, 3, 4, 5, 6),    // 6
        number!(0, 2, 5),             // 7
        number!(0, 1, 2, 3, 4, 5, 6), // 8
        number!(0, 1, 2, 3, 5, 6),    // 9
    ]
}

#[derive(Clone, Copy, Debug)]
struct Segment(u8);

impl Segment {
    fn score(self, nums: &[Number; 10]) -> usize {
        let mut three = 0;
        let mut five = 0;
        let mut six = 0;
        for num in nums.iter() {
            if num.has_segment(self) {
                match num.count_segments() {
                    3 => three += 1,
                    5 => five += 1,
                    6 => six += 1,
                    _ => {}
                }
            }
        }

        three + (five << 1) + (six << 3)
    }

    fn deduce(self, nums: &[Number; 10]) -> Option<Segment> {
        let target_score = self.score(nums);
        let real = real_nums();
        for s in 0..7 {
            let test_seg = Segment(s);
            if target_score == test_seg.score(&real) {
                return Some(test_seg);
            }
        }
        None
    }
}

impl TryFrom<char> for Segment {
    type Error = anyhow::Error;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        let idx = match c {
            'a' => 0,
            'b' => 1,
            'c' => 2,
            'd' => 3,
            'e' => 4,
            'f' => 5,
            'g' => 6,
            _ => bail!("invalid char for segment: {}", c),
        };
        Ok(Segment(idx))
    }
}
