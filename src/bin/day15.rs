use aoc::{aoc_main, Result};

fn main() {
    aoc_main(15, parse::input, part_one, part_two).unwrap()
}

fn part_one(cavern: Cavern) -> Result<u64> {
    Ok(cavern.find_min_risk_level())
}

fn part_two(mut cavern: Cavern) -> Result<u64> {
    cavern.expand(5, 5);
    Ok(cavern.find_min_risk_level())
}

#[derive(Debug, Clone)]
struct Cavern {
    width: usize,
    risk_levels: Vec<u8>,
}

impl Cavern {
    fn find_min_risk_level(&self) -> u64 {
        let mut lowest = vec![None; self.risk_levels.len()];

        lowest[0] = Some(0);

        loop {
            let mut changed = false;

            for i in 0..lowest.len() {
                if let Some(min_neighbor) = find_min_neighbor(&lowest, self.width, i) {
                    let new_low = min_neighbor + u64::from(self.risk_levels[i]);
                    let is_lower = match lowest[i] {
                        Some(low) => new_low < low,
                        None => true,
                    };

                    if is_lower {
                        lowest[i] = Some(new_low);
                        changed = true;
                    }
                }
            }

            if !changed {
                break;
            }
        }

        lowest.last().unwrap().unwrap()
    }

    fn height(&self) -> usize {
        self.risk_levels.len() / self.width
    }

    fn expand(&mut self, rows: usize, cols: usize) {
        let mut new_map = vec![0u8; self.risk_levels.len() * rows * cols];

        let rstride = self.width * cols;
        let xstride = self.risk_levels.len() * cols;
        let ystride = self.width;
        for r in 0..self.height() {
            for c in 0..self.width {
                for x in 0..rows {
                    for y in 0..cols {
                        let old_val = self.risk_levels[r * self.width + c];
                        let new_val = (((old_val - 1) + x as u8 + y as u8) % 9) + 1;
                        new_map[x * xstride + y * ystride + r * rstride + c] = new_val;
                    }
                }
            }
        }
        self.width *= 5;
        self.risk_levels = new_map;
    }
}

fn find_min_neighbor(lowest: &[Option<u64>], width: usize, idx: usize) -> Option<u64> {
    let left = |i| if i % width == 0 { None } else { Some(i - 1) };
    let right = |i| {
        if (i + 1) % width == 0 {
            None
        } else {
            Some(i + 1)
        }
    };
    let up = |i| if i < width { None } else { Some(i - width) };
    let down = |i| {
        if i + width < lowest.len() {
            Some(i + width)
        } else {
            None
        }
    };

    let neighbors = [
        left(idx).and_then(|i| lowest[i]),
        right(idx).and_then(|i| lowest[i]),
        up(idx).and_then(|i| lowest[i]),
        down(idx).and_then(|i| lowest[i]),
    ];

    neighbors.into_iter().flatten().min()
}

mod parse {
    use super::*;
    use anyhow::anyhow;

    pub(super) fn input(s: &str) -> Result<Cavern> {
        let mut risk_levels = Vec::new();
        let mut width = None;
        for line in s.trim().lines() {
            let bytes = line.trim().bytes();
            if let Some(w) = width {
                if w != bytes.len() {
                    return Err(anyhow!("Invalid line length"));
                }
            } else {
                width = Some(bytes.len());
            }

            risk_levels.extend(bytes.map(|b| b - b'0'));
        }

        Ok(Cavern {
            width: width.unwrap_or(1),
            risk_levels,
        })
    }
}
