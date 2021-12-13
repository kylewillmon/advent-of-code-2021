use anyhow::anyhow;
use aoc::{aoc_main, Result};
use std::collections::HashMap;

fn main() {
    aoc_main(12, |s| Ok(s.to_owned()), part_one, part_two).unwrap()
}

fn part_one(input: String) -> Result<usize> {
    let map = CaveMap::from_input(&input)?;
    Ok(map.count_paths("start", "end", |p, n| !is_small(n) || !p.contains(&n)))
}

fn part_two(input: String) -> Result<usize> {
    let map = CaveMap::from_input(&input)?;
    Ok(map.count_paths("start", "end", |p, n| {
        !is_small(n) || !p.contains(&n) || !has_double_small(p)
    }))
}

fn has_double_small(path: &[&str]) -> bool {
    for i in 0..path.len() {
        if is_small(path[i]) {
            for j in (i + 1)..path.len() {
                if path[i] == path[j] {
                    return true;
                }
            }
        }
    }
    false
}

#[derive(Debug, Clone)]
struct CaveMap<'a>(HashMap<&'a str, Vec<&'a str>>);

impl<'a> CaveMap<'a> {
    fn from_input(input: &'a str) -> Result<Self> {
        let mut map: HashMap<&'a str, Vec<&'a str>> = HashMap::new();

        for line in input.trim().lines() {
            let mut link = line.trim().splitn(2, '-');
            let a = link.next().ok_or_else(|| anyhow!("invalid line"))?;
            let b = link.next().ok_or_else(|| anyhow!("invalid line"))?;

            map.entry(a).or_default().push(b);
            map.entry(b).or_default().push(a);
        }

        Ok(CaveMap(map))
    }

    fn count_paths<F>(&self, from: &'a str, to: &str, mut allowed: F) -> usize
    where
        F: FnMut(&[&'a str], &'a str) -> bool,
    {
        let mut count = 0;
        let mut path: Vec<&'a str> = vec![from];
        let mut nexts = vec![self.0.get(from).cloned().unwrap_or_default()];

        while let Some(next) = nexts.last_mut() {
            match next.pop() {
                Some(n) => {
                    if n == to {
                        count += 1;
                    } else if n != from && allowed(&path, n) {
                        path.push(n);
                        nexts.push(self.0.get(n).cloned().unwrap_or_default())
                    }
                }
                None => {
                    nexts.pop();
                    path.pop();
                    continue;
                }
            };
        }

        count
    }
}

fn is_small(cave: &str) -> bool {
    cave.chars().all(char::is_lowercase)
}
