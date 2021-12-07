use aoc::{aoc_main, Result};

fn main() {
    aoc_main(
        6,
        |s| {
            let v: Result<Vec<u8>, _> = s.split(',').map(|s| s.trim().parse::<u8>()).collect();
            Ok(v?)
        },
        |start| part_one(start),
        |start| part_two(start),
    )
    .unwrap()
}

fn part_one(start: Vec<u8>) -> Result<usize> {
    let mut counts = [0usize; 9];

    for fish in start.into_iter() {
        counts[fish as usize] += 1;
    }

    for _ in 0..80 {
        let new_fish = counts[0];
        counts.rotate_left(1);
        counts[6] += new_fish;
    }

    Ok(counts.into_iter().sum())
}

fn part_two(start: Vec<u8>) -> Result<usize> {
    let mut counts = [0usize; 9];

    for fish in start.into_iter() {
        counts[fish as usize] += 1;
    }

    for _ in 0..256 {
        let new_fish = counts[0];
        counts.rotate_left(1);
        counts[6] += new_fish;
    }

    Ok(counts.into_iter().sum())
}
