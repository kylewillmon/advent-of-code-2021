use aoc::{aoc_main, Result};

fn main() {
    aoc_main(
        7,
        |s| {
            let v: Result<Vec<usize>, _> =
                s.split(',').map(|s| s.trim().parse::<usize>()).collect();
            Ok(v?)
        },
        part_one,
        part_two,
    )
    .unwrap()
}

fn calc_min<Map: Clone + Fn(usize) -> usize>(start: Vec<usize>, cost_map: Map) -> Result<usize> {
    let max = start.iter().cloned().max().unwrap();

    // Start with the cost to send them all to zero.
    let mut min: usize = start.iter().cloned().map(cost_map.clone()).sum();

    for pos in 1..=max {
        let cost: usize = start.iter().cloned().map(|s| cost_map(diff(s, pos))).sum();
        if cost < min {
            min = cost;
        }
    }

    Ok(min)
}

fn part_one(start: Vec<usize>) -> Result<usize> {
    calc_min(start, |d| d)
}

fn part_two(start: Vec<usize>) -> Result<usize> {
    calc_min(start, |d| d * (d + 1) / 2)
}

fn diff(a: usize, b: usize) -> usize {
    if a < b {
        b - a
    } else {
        a - b
    }
}
