use aoc::{aoc_main, Result};

use anyhow::anyhow;

fn main() {
    aoc_main(
        3,
        |s| {
            let v: Vec<String> = s.lines().map(|l| l.to_owned()).collect();
            Ok(v)
        },
        part_one,
        part_two,
    )
    .unwrap()
}

fn part_one(input: Vec<String>) -> Result<i64> {
    let bitwidth = input.first().map(|s| s.len()).unwrap_or(0);

    let mut gamma = 0;
    let mut epsilon = 0;

    for bit in 0..bitwidth {
        gamma <<= 1;
        epsilon <<= 1;

        match most_common(&input, bit) {
            Bit::One => gamma |= 1,
            Bit::Zero => epsilon |= 1,
        }
    }

    Ok(epsilon * gamma)
}

fn part_two(input: Vec<String>) -> Result<i64> {
    let bitwidth = input.first().map(|s| s.len()).unwrap_or(0);

    let mut oxy = input.clone();
    let mut co2 = input;

    for bit in 0..bitwidth {
        if oxy.len() > 1 {
            let oxy_crit = most_common(&oxy, bit).as_char();
            oxy.retain(|l| l.chars().nth(bit).filter(|c| *c == oxy_crit).is_some());
        }

        if co2.len() > 1 {
            let co2_crit = least_common(&co2, bit).as_char();
            co2.retain(|l| l.chars().nth(bit).filter(|c| *c == co2_crit).is_some());
        }
    }

    let oxy = oxy
        .first()
        .ok_or(anyhow!("no oxygen generator ratings left"))?;
    let co2 = co2.first().ok_or(anyhow!("no co2 scrubber ratings left"))?;

    let oxy = i64::from_str_radix(oxy, 2)?;
    let co2 = i64::from_str_radix(co2, 2)?;

    Ok(oxy * co2)
}

#[derive(Clone, Copy, Debug)]
enum Bit {
    One,
    Zero,
}

impl Bit {
    fn as_char(&self) -> char {
        match self {
            Bit::One => '1',
            Bit::Zero => '0',
        }
    }
}

fn most_common(input: &[String], bit: usize) -> Bit {
    let ones = input
        .iter()
        .filter(|l| l.chars().nth(bit).filter(|c| *c == '1').is_some())
        .count();

    if ones + ones >= input.len() {
        Bit::One
    } else {
        Bit::Zero
    }
}

fn least_common(input: &[String], bit: usize) -> Bit {
    match most_common(input, bit) {
        Bit::One => Bit::Zero,
        Bit::Zero => Bit::One,
    }
}
