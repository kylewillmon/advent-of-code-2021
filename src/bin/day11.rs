use aoc::{aoc_main, Result};

fn main() {
    aoc_main(
        11,
        |s| {
            Ok(Cavern(
                s.trim()
                    .lines()
                    .map(|l| l.trim().bytes().map(|b| b - b'0'))
                    .flatten()
                    .collect(),
            ))
        },
        part_one,
        part_two,
    )
    .unwrap()
}

fn part_one(mut grid: Cavern) -> Result<usize> {
    let mut total = 0;
    for _ in 0..100 {
        grid.increment_all();
        loop {
            let flashes = grid.calc_flashes();
            if flashes == 0 {
                break;
            }
            total += flashes;
        }
    }
    Ok(total)
}

fn part_two(mut grid: Cavern) -> Result<usize> {
    let mut step = 1;
    loop {
        grid.increment_all();
        let mut total = 0;
        loop {
            let flashes = grid.calc_flashes();
            if flashes == 0 {
                break;
            }
            total += flashes;
        }
        if total == 100 {
            break;
        }
        step += 1;
    }
    Ok(step)
}

#[derive(Clone, Debug)]
struct Cavern(Vec<u8>);

impl Cavern {
    fn increment_all(&mut self) {
        for oct in self.0.iter_mut() {
            *oct += 1;
        }
    }

    fn calc_flashes(&mut self) -> usize {
        let mut count = 0;
        for i in 0..self.0.len() {
            if self.0[i] > 9 {
                count += 1;
                self.0[i] = 0;
                let neighbors = [
                    left(i),
                    right(i),
                    up(i),
                    down(i),
                    up(i).and_then(left),
                    up(i).and_then(right),
                    down(i).and_then(left),
                    down(i).and_then(right),
                ];
                for i in neighbors.into_iter().flatten() {
                    if self.0[i] != 0 {
                        self.0[i] += 1;
                    }
                }
            }
        }
        count
    }
}

fn left(idx: usize) -> Option<usize> {
    if idx % 10 == 0 {
        None
    } else {
        Some(idx - 1)
    }
}

fn right(idx: usize) -> Option<usize> {
    if idx % 10 == 9 {
        None
    } else {
        Some(idx + 1)
    }
}

fn up(idx: usize) -> Option<usize> {
    if idx < 10 {
        None
    } else {
        Some(idx - 10)
    }
}

fn down(idx: usize) -> Option<usize> {
    if idx >= 90 {
        None
    } else {
        Some(idx + 10)
    }
}
