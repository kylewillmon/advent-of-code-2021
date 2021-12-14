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
        |_| Ok(0),
    )
    .unwrap()
}

fn solve(mut template: PolymerTemplate, rules: PairInsertionRules, steps: usize) -> Result<usize> {
    for _ in 0..steps {
        template = template.run_insertion_rules(&rules);
    }
    let counts = template.to_counts();

    let val = match (counts.first(), counts.last()) {
        (Some(first), Some(last)) => last - first,
        _ => 0,
    };
    Ok(val)
}

#[derive(Debug, Clone)]
struct PolymerTemplate(Vec<u8>);

impl PolymerTemplate {
    fn run_insertion_rules(&self, rules: &PairInsertionRules) -> Self {
        let mut t: Vec<u8> = self
            .0
            .windows(2)
            .map(|pair| {
                let mut arr = [pair[0], pair[0]];
                if let Some(p) = rules.0.get(pair) {
                    arr[1] = *p;
                    arr.into_iter()
                } else {
                    let mut iter = arr.into_iter();
                    iter.next();
                    iter
                }
            })
            .flatten()
            .collect();
        if let Some(p) = self.0.last() {
            t.push(*p);
        }
        PolymerTemplate(t)
    }

    fn to_counts(self) -> Vec<usize> {
        let mut polys = self.0;
        polys.sort_unstable();
        let mut polys = polys.into_iter();

        let mut counts = Vec::new();
        if let Some(first) = polys.next() {
            let mut last = first;
            let mut count = 1;
            for p in polys {
                if p == last {
                    count += 1;
                } else {
                    counts.push(count);
                    last = p;
                    count = 1;
                }
            }
        }

        counts.sort_unstable();

        counts
    }
}

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
