use aoc::{aoc_main, Result};
use nom::Finish;

fn main() {
    aoc_main(
        5,
        |s| {
            let (_, lines) = parse::lines(s).finish().unwrap();
            Ok(lines)
        },
        |lines| part_one(lines),
        |lines| part_two(lines),
    )
    .unwrap()
}

fn part_one(lines: Vec<Line>) -> Result<usize> {
    let mut grid = vec![0u8; 1000000];

    for l in lines.into_iter() {
        if l.start.x == l.end.x {
            let x = l.start.x;
            let (s, e) = in_order(l.start.y, l.end.y);
            for y in s..=e {
                let idx = (x as usize) * 1000 + (y as usize);
                grid[idx] += 1;
            }
        }

        if l.start.y == l.end.y {
            let y = l.start.y;
            let (s, e) = in_order(l.start.x, l.end.x);
            for x in s..=e {
                let idx = (x as usize) * 1000 + (y as usize);
                grid[idx] += 1;
            }
        }
    }

    Ok(grid.into_iter().filter(|c| *c > 1).count())
}

fn part_two(lines: Vec<Line>) -> Result<usize> {
    let mut grid = vec![0u8; 1000000];

    for l in lines.into_iter() {
        if l.start.x == l.end.x {
            let x = l.start.x;
            let (s, e) = in_order(l.start.y, l.end.y);
            for y in s..=e {
                let idx = (x as usize) * 1000 + (y as usize);
                grid[idx] += 1;
            }
            continue;
        }

        if l.start.y == l.end.y {
            let y = l.start.y;
            let (s, e) = in_order(l.start.x, l.end.x);
            for x in s..=e {
                let idx = (x as usize) * 1000 + (y as usize);
                grid[idx] += 1;
            }
            continue;
        }

        if diff(l.start.x, l.end.x) == diff(l.start.y, l.end.y) {
            let diff = diff(l.start.x, l.end.x);
            for d in 0..=diff {
                let x = if l.end.x > l.start.x {
                    l.start.x + d
                } else {
                    l.start.x - d
                };
                let y = if l.end.y > l.start.y {
                    l.start.y + d
                } else {
                    l.start.y - d
                };

                let idx = (x as usize) * 1000 + (y as usize);
                grid[idx] += 1;
            }
        }
    }

    Ok(grid.into_iter().filter(|c| *c > 1).count())
}

fn in_order(a: u16, b: u16) -> (u16, u16) {
    if a < b {
        (a, b)
    } else {
        (b, a)
    }
}

fn diff(a: u16, b: u16) -> u16 {
    if a > b {
        a - b
    } else {
        b - a
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Point {
    x: u16,
    y: u16,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Line {
    start: Point,
    end: Point,
}

mod parse {
    use super::*;

    use nom::bytes::complete::tag;
    use nom::character::complete::line_ending;
    use nom::multi::many1;
    use nom::sequence::{separated_pair, terminated};
    use nom::IResult;

    fn point(s: &str) -> IResult<&str, Point> {
        let (s, (x, y)) = separated_pair(
            nom::character::complete::u16,
            tag(","),
            nom::character::complete::u16,
        )(s)?;
        Ok((s, Point { x, y }))
    }

    fn line(s: &str) -> IResult<&str, Line> {
        let (s, (start, end)) =
            terminated(separated_pair(point, tag(" -> "), point), line_ending)(s)?;
        Ok((s, Line { start, end }))
    }

    pub(super) fn lines(s: &str) -> IResult<&str, Vec<Line>> {
        many1(line)(s)
    }
}
