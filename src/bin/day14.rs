use std::collections::HashMap;

use anyhow::anyhow;
use aoc::{aoc_main, Result};

fn main() {
    use nom::Finish;
    aoc_main(
        14,
        |s| {
            let (_, out) = parse::input(s)
                .finish()
                .map_err(|e| anyhow!("failed to parse input: {:?}", e))?;
            Ok(out)
        },
        |(template, rules)| solve(template, rules, 10),
        |(template, rules)| solve(template, rules, 40),
    )
    .unwrap()
}

fn solve(template: PolymerTemplate, rules: PairInsertionRules, steps: usize) -> Result<usize> {
    let mut memory = HashMap::new();
    let mut counts = HashMap::new();

    for pair in template.0.windows(2) {
        add_counts(
            &mut counts,
            get_counts(pair.try_into().unwrap(), steps, &rules, &mut memory),
        );
    }

    if let Some(poly) = template.0.last() {
        *counts.entry(*poly).or_insert(0) += 1;
    }

    let mut counts: Vec<usize> = counts.values().copied().collect();

    counts.sort_unstable();

    let val = match (counts.first(), counts.last()) {
        (Some(first), Some(last)) => last - first,
        _ => 0,
    };
    Ok(val)
}

fn get_counts(
    pair: [u8; 2],
    steps: usize,
    rules: &PairInsertionRules,
    memory: &mut HashMap<([u8; 2], usize), HashMap<u8, usize>>,
) -> HashMap<u8, usize> {
    if let Some(val) = memory.get(&(pair, steps)) {
        return val.clone();
    }

    let poly = match (steps, rules.0.get(&pair)) {
        (0, _) | (_, None) => return [(pair[0], 1)].into_iter().collect(),
        (_, Some(poly)) => *poly,
    };

    let sub_pair = [pair[0], poly];
    let mut counts = get_counts(sub_pair, steps - 1, rules, memory);

    let sub_pair = [poly, pair[1]];

    add_counts(&mut counts, get_counts(sub_pair, steps - 1, rules, memory));

    memory.insert((pair, steps), counts.clone());

    counts
}

fn add_counts(dest: &mut HashMap<u8, usize>, src: HashMap<u8, usize>) {
    for (k, v) in src.into_iter() {
        *dest.entry(k).or_insert(0) += v;
    }
}

#[derive(Debug, Clone)]
struct PolymerTemplate(Vec<u8>);

#[derive(Debug, Clone)]
struct PairInsertionRules(HashMap<[u8; 2], u8>);

mod parse {
    use nom::{
        bytes::complete::{tag, take, take_while},
        character::{complete::line_ending, is_alphanumeric},
        combinator::map,
        multi::many0,
        sequence::{separated_pair, terminated},
        IResult,
    };

    use super::*;

    fn template(i: &[u8]) -> IResult<&[u8], PolymerTemplate> {
        map(
            terminated(take_while(is_alphanumeric), line_ending),
            |p: &[u8]| PolymerTemplate(p.to_vec()),
        )(i)
    }

    fn pair_insertion(i: &[u8]) -> IResult<&[u8], ([u8; 2], u8)> {
        let (i, (pair, insert)) = terminated(
            separated_pair(take(2usize), tag(" -> "), take(1usize)),
            line_ending,
        )(i)?;
        Ok((i, (pair.try_into().unwrap(), insert[0])))
    }

    pub(super) fn input(i: &str) -> IResult<&[u8], (PolymerTemplate, PairInsertionRules)> {
        separated_pair(
            template,
            line_ending,
            map(many0(pair_insertion), |v| {
                PairInsertionRules(v.into_iter().collect())
            }),
        )(i.as_bytes())
    }
}
