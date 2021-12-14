use std::fmt::Display;

use anyhow::anyhow;
use aoc::{aoc_main, Result};

fn main() {
    use nom::Finish;
    aoc_main(
        13,
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

fn part_one(input: (Paper, Vec<Fold>)) -> Result<usize> {
    let (mut paper, folds) = input;
    paper.fold(folds.first().unwrap().clone());

    Ok(paper.0.len())
}

fn part_two(input: (Paper, Vec<Fold>)) -> Result<usize> {
    let (mut paper, folds) = input;

    for f in folds {
        paper.fold(f);
    }

    println!("{}", paper);

    Ok(paper.0.len())
}

#[derive(Debug, Clone)]
enum Fold {
    Up(u32),
    Left(u32),
}

#[derive(Debug, Clone)]
struct Paper(Vec<(u32, u32)>);

impl Paper {
    fn fold(&mut self, fold: Fold) {
        for item in self.0.iter_mut() {
            let (x, y) = *item;
            match fold {
                Fold::Up(y0) if y > y0 => {
                    item.1 = y0 - (y - y0);
                }
                Fold::Left(x0) if x > x0 => {
                    item.0 = x0 - (x - x0);
                }
                _ => {}
            }
        }
        self.0.sort_unstable();
        self.0.dedup();
    }
}

impl Display for Paper {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut width = 0;
        let mut lines = 0;
        for &(x, y) in &self.0 {
            if x > width {
                width = x
            }
            if y > lines {
                lines = y
            }
        }
        let width = (width + 2) as usize;
        let lines = (lines + 1) as usize;

        let mut out = vec![b'.'; width * lines];
        for &(x, y) in &self.0 {
            out[(y as usize) * width + (x as usize)] = b'#';
        }
        for i in 0..lines {
            out[i * width + (width - 1)] = b'\n';
        }

        write!(f, "{}", unsafe { std::str::from_utf8_unchecked(&out) })
    }
}

mod parse {
    use super::*;
    use nom::{
        branch::alt,
        bytes::complete::tag,
        character::complete::line_ending,
        multi::many0,
        sequence::{delimited, separated_pair, terminated},
        IResult,
    };

    fn point(i: &str) -> IResult<&str, (u32, u32)> {
        terminated(
            separated_pair(
                nom::character::complete::u32,
                tag(","),
                nom::character::complete::u32,
            ),
            line_ending,
        )(i)
    }

    fn paper(i: &str) -> IResult<&str, Paper> {
        let (i, vec) = many0(point)(i)?;
        Ok((i, Paper(vec)))
    }

    fn fold(i: &str) -> IResult<&str, Fold> {
        let (i, letter) = delimited(tag("fold along "), alt((tag("x"), tag("y"))), tag("="))(i)?;
        let (i, val) = terminated(nom::character::complete::u32, line_ending)(i)?;
        let fold = match letter {
            "y" => Fold::Up(val),
            "x" => Fold::Left(val),
            _ => unreachable!(),
        };
        Ok((i, fold))
    }

    pub(super) fn input(i: &str) -> IResult<&str, (Paper, Vec<Fold>)> {
        separated_pair(paper, line_ending, many0(fold))(i)
    }
}
