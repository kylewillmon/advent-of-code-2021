use anyhow::anyhow;
use aoc::{aoc_main, Result};

fn main() {
    use nom::Finish;
    aoc_main(
        18,
        |s| {
            let (_, out) = parse::input(s.trim())
                .finish()
                .map_err(|e| anyhow!("failed to parse input: {:?}", e))?;
            Ok(out)
        },
        part_one,
        part_two,
    )
    .unwrap()
}

fn part_one(nums: Vec<SnailfishNumber>) -> Result<u64> {
    let mut nums = nums.into_iter();
    let first = nums.next().unwrap();
    let res = nums.fold(first, add);
    Ok(res.magnitude())
}

fn part_two(mut nums: Vec<SnailfishNumber>) -> Result<u64> {
    let mut max = 0;

    while let Some(a) = nums.pop() {
        for b in nums.iter().cloned() {
            let c = add(a.clone(), b.clone()).magnitude();
            if c > max {
                max = c;
            }
            let c = add(b, a.clone()).magnitude();
            if c > max {
                max = c;
            }
        }
    }

    Ok(max)
}

#[derive(Debug, Clone, Copy)]
enum Token {
    Open,
    Close,
    Literal(u8),
    Comma,
}

#[derive(Debug, Clone)]
struct SnailfishNumber(Vec<Token>);

impl std::fmt::Display for SnailfishNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for tok in self.0.iter() {
            match tok {
                Token::Open => write!(f, "[")?,
                Token::Close => write!(f, "]")?,
                Token::Literal(val) => write!(f, "{}", val)?,
                Token::Comma => write!(f, ",")?,
            }
        }
        Ok(())
    }
}

fn add(left: SnailfishNumber, right: SnailfishNumber) -> SnailfishNumber {
    let mut res = left;
    res.0.insert(0, Token::Open);
    res.0.push(Token::Comma);
    res.0.extend(right.0.into_iter());
    res.0.push(Token::Close);

    res.reduce()
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum ReduceState {
    Done,
    NotDone,
}

impl SnailfishNumber {
    fn reduce_step(self) -> (Self, ReduceState) {
        let mut depth = 0;
        for i in 0..self.0.len() {
            match self.0[i] {
                Token::Open => {
                    depth += 1;
                }
                Token::Close => {
                    depth -= 1;
                }
                _ => {}
            }
            if depth > 4 {
                return (self.explode(i), ReduceState::NotDone);
            }
        }

        for i in 0..self.0.len() {
            if let Token::Literal(val) = self.0[i] {
                if val > 9 {
                    return (self.split(i), ReduceState::NotDone);
                }
            }
        }
        (self, ReduceState::Done)
    }

    fn reduce(self) -> Self {
        let mut num = self;
        let mut state = ReduceState::NotDone;
        while state != ReduceState::Done {
            let (n, s) = num.reduce_step();
            num = n;
            state = s;
        }
        num
    }

    fn split(mut self, i: usize) -> Self {
        let val = if let Token::Literal(val) = self.0[i] {
            val
        } else {
            panic!("invalid split");
        };
        let (val, extra) = (val / 2, val % 2);
        self.0[i] = Token::Close;
        self.0.insert(i, Token::Literal(val + extra));
        self.0.insert(i, Token::Comma);
        self.0.insert(i, Token::Literal(val));
        self.0.insert(i, Token::Open);
        self
    }

    fn explode(mut self, i: usize) -> Self {
        let (left, right) = match self.0[i..i + 5] {
            [Token::Open, Token::Literal(left), Token::Comma, Token::Literal(right), Token::Close] => {
                (left, right)
            }
            _ => panic!("invalid explode"),
        };

        self.0[i..].rotate_left(4);
        self.0.truncate(self.0.len() - 4);
        self.0[i] = Token::Literal(0);

        for tok in self.0[..i].iter_mut().rev() {
            if let Token::Literal(ref mut val) = tok {
                *val += left;
                break;
            }
        }

        for tok in self.0[i + 1..].iter_mut() {
            if let Token::Literal(ref mut val) = tok {
                *val += right;
                break;
            }
        }

        self
    }

    fn magnitude(&self) -> u64 {
        let mut base = 1;
        let mut mul = 1;
        let mut oldbase = Vec::new();
        let mut acc = 0;

        for tok in self.0.iter().copied() {
            match tok {
                Token::Open => {
                    oldbase.push(base);
                    base *= mul;
                    mul = 3;
                }
                Token::Close => {
                    base = oldbase.pop().unwrap();
                }
                Token::Comma => {
                    mul = 2;
                }
                Token::Literal(val) => {
                    acc += (val as u64) * mul * base;
                }
            }
        }

        acc
    }
}

mod parse {
    use nom::{
        branch::alt,
        bytes::complete::tag,
        character::complete::{digit1, line_ending, space0},
        combinator::map_res,
        multi::{many1, separated_list1},
        sequence::preceded,
        IResult,
    };

    use super::*;

    fn token(s: &str) -> IResult<&str, Token> {
        map_res(
            alt((tag("["), tag("]"), tag(","), digit1)),
            |t| -> Result<Token> {
                Ok(match t {
                    "[" => Token::Open,
                    "]" => Token::Close,
                    "," => Token::Comma,
                    _ => Token::Literal(t.parse()?),
                })
            },
        )(s)
    }

    fn snailfish_number(s: &str) -> IResult<&str, SnailfishNumber> {
        let (s, num) = many1(preceded(space0, token))(s)?;
        Ok((s, SnailfishNumber(num)))
    }

    pub(super) fn input(s: &str) -> IResult<&str, Vec<SnailfishNumber>> {
        separated_list1(line_ending, snailfish_number)(s)
    }
}
