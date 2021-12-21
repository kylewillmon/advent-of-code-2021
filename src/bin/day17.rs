use std::ops::RangeInclusive;

use anyhow::anyhow;

use aoc::{aoc_main, Result};

fn main() {
    use nom::Finish;
    aoc_main(
        17,
        |s| {
            let (_, out) = parse::input(s)
                .finish()
                .map_err(|e| anyhow!("failed to parse input: {:?}", e))?;
            Ok(out)
        },
        part_one,
        part_two,
    )
    .unwrap()
}

fn part_one(target: TargetArea) -> Result<i64> {
    let y_min = *target.y_range.start();
    let y_start = -y_min - 1;
    Ok(y_start * (y_start + 1) / 2)
}

fn part_two(target: TargetArea) -> Result<i64> {
    let x_max = *target.x_range.end();
    let y_min = *target.y_range.start();
    let mut count = 0;
    for x in 0..=x_max {
        for y in y_min..=-y_min {
            let probe = Probe::with_velocity(x, y);
            if probe.will_hit(&target) {
                count += 1;
            }
        }
    }

    Ok(count)
}

#[derive(Clone, Debug)]
struct TargetArea {
    x_range: RangeInclusive<i64>,
    y_range: RangeInclusive<i64>,
}

#[derive(Clone, Debug)]
struct Probe {
    x: i64,
    y: i64,
    x_velocity: i64,
    y_velocity: i64,
}

impl Probe {
    fn with_velocity(x_velocity: i64, y_velocity: i64) -> Self {
        Probe {
            x: 0,
            y: 0,
            x_velocity,
            y_velocity,
        }
    }

    fn step(&mut self) {
        use std::cmp::Ordering;
        self.x += self.x_velocity;
        self.y += self.y_velocity;
        match Ord::cmp(&self.x_velocity, &0) {
            Ordering::Greater => {
                self.x_velocity -= 1;
            }
            Ordering::Less => {
                self.x_velocity += 1;
            }
            _ => {}
        }
        self.y_velocity -= 1;
    }

    fn will_hit(self, target: &TargetArea) -> bool {
        let mut probe = self;
        while probe.y_velocity >= 0 || probe.y >= *target.y_range.start() {
            if target.x_range.contains(&probe.x) && target.y_range.contains(&probe.y) {
                return true;
            }
            probe.step();
        }
        false
    }
}

mod parse {
    use nom::{bytes::complete::tag, IResult};

    use super::*;

    pub(super) fn input(input: &str) -> IResult<&str, TargetArea> {
        let (input, _) = tag("target area: x=")(input)?;
        let (input, x_min) = nom::character::complete::i64(input)?;
        let (input, _) = tag("..")(input)?;
        let (input, x_max) = nom::character::complete::i64(input)?;
        let (input, _) = tag(", y=")(input)?;
        let (input, y_min) = nom::character::complete::i64(input)?;
        let (input, _) = tag("..")(input)?;
        let (input, y_max) = nom::character::complete::i64(input)?;

        Ok((
            input,
            TargetArea {
                x_range: (x_min..=x_max),
                y_range: (y_min..=y_max),
            },
        ))
    }
}
