use anyhow::anyhow;

use aoc::{aoc_main, Result};

fn main() {
    aoc_main(10, |s| Ok(s.to_owned()), part_one, part_two).unwrap()
}

fn part_one(s: String) -> Result<i64> {
    let mut sum = 0;

    for line in s.lines() {
        if let LineState::Corrupted(score) = line_state(line)? {
            sum += score;
        }
    }

    Ok(sum)
}

fn part_two(s: String) -> Result<i64> {
    let mut scores = Vec::new();

    for line in s.lines() {
        if let LineState::Incomplete(score) = line_state(line)? {
            scores.push(score)
        }
    }

    scores.sort_unstable();

    Ok(scores[scores.len() / 2])
}

fn line_state(line: &str) -> Result<LineState> {
    let mut closing: Vec<Brace> = Vec::new();

    for c in line.chars() {
        let b = Brace::try_from(c)?;
        if b.is_opening() {
            closing.push(b.as_closing());
        } else if closing.pop() != Some(b) {
            return Ok(LineState::Corrupted(corrupted_score(b)));
        }
    }

    if closing.is_empty() {
        Ok(LineState::Valid)
    } else {
        Ok(LineState::Incomplete(autocomplete_score(closing)))
    }
}

#[derive(Clone, PartialEq, Eq)]
enum LineState {
    Valid,
    Corrupted(i64),
    Incomplete(i64),
}

fn corrupted_score(b: Brace) -> i64 {
    match b.0 {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => 0,
    }
}

fn autocomplete_score(stack: Vec<Brace>) -> i64 {
    let mut val = 0;
    for b in stack.into_iter().rev() {
        val *= 5;
        val += match b.0 {
            ')' => 1,
            ']' => 2,
            '}' => 3,
            '>' => 4,
            _ => 0,
        };
    }
    val
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Brace(char);

static OPEN_BRACE: &str = "([{<";
static CLOSE_BRACE: &str = ")]}>";

impl Brace {
    fn is_opening(&self) -> bool {
        OPEN_BRACE.contains(self.0)
    }

    fn as_closing(&self) -> Self {
        if let Some((idx, _)) = OPEN_BRACE.chars().enumerate().find(|&(_, c)| c == self.0) {
            Self(CLOSE_BRACE.chars().nth(idx).unwrap())
        } else {
            *self
        }
    }
}

impl TryFrom<char> for Brace {
    type Error = anyhow::Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        if OPEN_BRACE.contains(value) || CLOSE_BRACE.contains(value) {
            Ok(Brace(value))
        } else {
            Err(anyhow!("invalid brace char: {}", value))
        }
    }
}
