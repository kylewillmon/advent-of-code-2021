use aoc::{aoc_main, Result};

fn main() {
    aoc_main(
        9,
        |s| {
            let mut lines = s.lines().peekable();
            let width = lines.peek().map(|l| l.len()).unwrap_or(1);

            let grid = lines
                .map(|l| l.bytes().map(|b| b - b'0'))
                .flatten()
                .collect();
            Ok(Grid::with_width(width, grid))
        },
        part_one,
        part_two,
    )
    .unwrap()
}

fn part_one(grid: Grid<u8>) -> Result<usize> {
    let mut count = 0;
    for col in 0..grid.width() {
        for row in 0..grid.height() {
            let val = grid.get(row, col).unwrap();

            if grid.neighbors(row, col).find(|v| *v <= val) == None {
                count += (1 + val) as usize;
            }
        }
    }

    Ok(count)
}

fn part_two(mut grid: Grid<u8>) -> Result<usize> {
    let mut val = 10u8;

    for row in 0..grid.height() {
        for col in 0..grid.width() {
            if grid.get(row, col).unwrap_or(9) < 9 {
                fill(&mut grid, row, col, val);

                assert!(val < u8::MAX);
                val += 1;
            }
        }
    }

    let mut top = [0usize; 3];

    for i in 10..val {
        let count = grid.count(i);

        if count > top[0] {
            top[0] = count;
            top.sort_unstable();
        }
    }

    Ok(top.into_iter().product())
}

fn fill(grid: &mut Grid<u8>, row: usize, col: usize, val: u8) {
    assert!(val > 9);

    let mut points = vec![(row, col)];
    while !points.is_empty() {
        let mut next = Vec::new();

        for (r, c) in points.into_iter() {
            if let Some(cell) = grid.get_mut(r, c) {
                *cell = val;
            }

            for (nr, nc) in AdjacentPoints::new(r, c) {
                if grid.get(nr, nc).unwrap_or(9) < 9 {
                    next.push((nr, nc));
                }
            }
        }

        points = next;
    }
}

#[derive(Debug, Clone)]
struct Grid<T> {
    width: usize,
    grid: Vec<T>,
}

impl<T> Grid<T> {
    fn with_width(width: usize, grid: Vec<T>) -> Self {
        assert!(width != 0);
        assert!(grid.len() % width == 0);

        Grid { width, grid }
    }

    fn width(&self) -> usize {
        self.width
    }

    fn height(&self) -> usize {
        self.grid.len() / self.width
    }

    fn get_mut(&mut self, row: usize, col: usize) -> Option<&mut T> {
        if col < self.width && row < self.height() {
            Some(&mut self.grid[row * self.width + col])
        } else {
            None
        }
    }
}

impl<T: Copy> Grid<T> {
    fn get(&self, row: usize, col: usize) -> Option<T> {
        if col < self.width && row < self.height() {
            Some(self.grid[row * self.width + col])
        } else {
            None
        }
    }

    fn neighbors(&self, row: usize, col: usize) -> Neighbors<'_, T> {
        Neighbors {
            points: AdjacentPoints::new(row, col),
            grid: self,
        }
    }
}

impl<T: Copy + PartialEq> Grid<T> {
    fn count(&self, val: T) -> usize {
        self.grid.iter().copied().filter(|v| *v == val).count()
    }
}

struct AdjacentPoints(std::vec::IntoIter<(usize, usize)>);

impl AdjacentPoints {
    fn new(row: usize, col: usize) -> Self {
        let mut points: Vec<(usize, usize)> = Vec::new();

        if let Some(r) = row.checked_sub(1) {
            points.push((r, col));
        }
        if let Some(r) = row.checked_add(1) {
            points.push((r, col));
        }
        if let Some(c) = col.checked_sub(1) {
            points.push((row, c));
        }
        if let Some(c) = col.checked_add(1) {
            points.push((row, c));
        }

        AdjacentPoints(points.into_iter())
    }
}

impl Iterator for AdjacentPoints {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
}

struct Neighbors<'a, T> {
    points: AdjacentPoints,
    grid: &'a Grid<T>,
}

impl<'a, T: Copy> Iterator for Neighbors<'a, T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        for (row, col) in self.points.by_ref() {
            if let Some(val) = self.grid.get(row, col) {
                return Some(val);
            }
        }
        None
    }
}
