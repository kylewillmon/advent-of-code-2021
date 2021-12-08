use std::str::FromStr;

use aoc::{aoc_main, Result};

use anyhow::{anyhow, ensure};
use nom::Finish;

fn main() {
    aoc_main(
        4,
        |s| Input::from_str(s),
        |i| i.part_one(),
        |i| i.part_two(),
    )
    .unwrap()
}

#[derive(Clone, Debug)]
struct Input {
    numbers: Vec<u8>,
    cards: Vec<BingoCard>,
}

impl Input {
    fn part_one(mut self) -> Result<i64> {
        for n in self.numbers.into_iter() {
            for c in self.cards.iter_mut() {
                c.mark(n);
                if c.has_bingo() {
                    return Ok(score(c, n));
                }
            }
        }
        Err(anyhow!("no bingo"))
    }

    fn part_two(mut self) -> Result<i64> {
        let mut last = None;
        for n in self.numbers.into_iter() {
            for c in self.cards.iter_mut() {
                c.mark(n);
                if c.has_bingo() {
                    last = Some((c.clone(), n));
                }
            }
            self.cards.retain(|c| !c.has_bingo());
        }

        if let Some((c, n)) = last {
            return Ok(score(&c, n));
        }

        Err(anyhow!("no bingo"))
    }
}

fn score(card: &BingoCard, num: u8) -> i64 {
    let sum: i64 = card
        .0
        .iter()
        .cloned()
        .filter(|s| !s.marked())
        .map(|s| s.num as i64)
        .sum();
    sum * (num as i64)
}

mod parse {
    use nom::bytes::complete::tag;
    use nom::character::complete::{line_ending, multispace0, space0};
    use nom::multi::{fill, separated_list1};
    use nom::sequence::terminated;
    use nom::IResult;

    use super::{BingoCard, Input};

    fn numbers(s: &str) -> IResult<&str, Vec<u8>> {
        terminated(
            separated_list1(tag(","), nom::character::complete::u8),
            line_ending,
        )(s)
    }

    fn bingo_card(s: &str) -> IResult<&str, BingoCard> {
        let mut card: [u8; 25] = [0; 25];
        let mut s = s;
        for c in card.chunks_mut(5) {
            let (rest, _) = terminated(
                fill(
                    |input: &str| {
                        let (input, _) = space0(input)?;
                        nom::character::complete::u8(input)
                    },
                    c,
                ),
                terminated(space0, line_ending),
            )(s)?;

            s = rest;
        }
        Ok((s, card.into_iter().collect()))
    }

    fn bingo_cards(s: &str) -> IResult<&str, Vec<BingoCard>> {
        separated_list1(line_ending, bingo_card)(s)
    }

    pub(super) fn input(s: &str) -> IResult<&str, Input> {
        let (s, numbers) = numbers(s)?;
        let (s, _) = line_ending(s)?;
        let (s, cards) = bingo_cards(s)?;
        let (s, _) = multispace0(s)?;

        Ok((s, Input { numbers, cards }))
    }
}

impl FromStr for Input {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self> {
        let (s, input) = match parse::input(s).finish() {
            Ok(o) => o,
            Err(nom::error::Error { input, code }) => {
                return Err(nom::error::Error {
                    input: input.to_string(),
                    code,
                }
                .into())
            }
        };
        ensure!(s.is_empty(), "Extra data in input file: {}", s);
        Ok(input)
    }
}

#[derive(Clone, Debug)]
struct BingoCard([BingoSquare; 25]);

impl BingoCard {
    fn has_bingo(&self) -> bool {
        for i in 0..5 {
            if self.row_has_bingo(i) {
                return true;
            }
            if self.col_has_bingo(i) {
                return true;
            }
        }
        false
    }

    fn row_has_bingo(&self, row: usize) -> bool {
        for col in 0..5 {
            if !self.0[row * 5 + col].marked() {
                return false;
            }
        }
        true
    }

    fn col_has_bingo(&self, col: usize) -> bool {
        for row in 0..5 {
            if !self.0[row * 5 + col].marked() {
                return false;
            }
        }
        true
    }

    fn mark(&mut self, num: u8) {
        for s in self.0.iter_mut() {
            if s.num == num {
                s.mark();
            }
        }
    }
}

impl FromIterator<u8> for BingoCard {
    fn from_iter<T: IntoIterator<Item = u8>>(iter: T) -> Self {
        let mut card = Vec::new();

        for i in iter.into_iter() {
            card.push(BingoSquare::new(i));
            if card.len() == 25 {
                break;
            }
        }

        card.resize(25, BingoSquare::new(0));

        BingoCard(card.try_into().unwrap())
    }
}

#[derive(Clone, Debug)]
struct BingoSquare {
    num: u8,
    state: SquareState,
}

impl BingoSquare {
    fn new(num: u8) -> Self {
        BingoSquare {
            num,
            state: SquareState::Unmarked,
        }
    }

    fn mark(&mut self) {
        self.state = SquareState::Marked;
    }

    fn marked(&self) -> bool {
        self.state == SquareState::Marked
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum SquareState {
    Unmarked,
    Marked,
}
