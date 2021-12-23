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

fn part_one(mut game: GameState) -> Result<u32> {
    let mut roll = 6;
    let mut num_rolls = 3;

    loop {
        game.0.turn(roll);
        if game.0.score >= 1000 {
            return Ok(game.1.score * num_rolls);
        }

        roll = (roll + 9) % 10;
        num_rolls += 3;

        game.1.turn(roll);
        if game.1.score >= 1000 {
            return Ok(game.0.score * num_rolls);
        }

        roll = (roll + 9) % 10;
        num_rolls += 3;
    }
}

fn part_two(game: GameState) -> Result<u64> {
    let mut games = vec![(0u64, 0u64); 10 * 10 * 21 * 21];

    for score1 in (0..21).rev() {
        for score2 in 0..=score1 {
            for pos1 in 0..10 {
                for pos2 in 0..10 {
                    let p1 = PlayerState {
                        pos: pos1,
                        score: score1,
                    };
                    let p2 = PlayerState {
                        pos: pos2,
                        score: score2,
                    };

                    let calc = |p1: &PlayerState, p2: &PlayerState, roll, mul| -> (u64, u64) {
                        let mut p1 = p1.clone();
                        p1.turn(roll);
                        let (a, b) = outcome(&GameState(p2.clone(), p1), &games);
                        assert!((a, b) != (0, 0));
                        (b * mul, a * mul)
                    };

                    let total = [
                        calc(&p1, &p2, 3, 1),
                        calc(&p1, &p2, 4, 3),
                        calc(&p1, &p2, 5, 6),
                        calc(&p1, &p2, 6, 7),
                        calc(&p1, &p2, 7, 6),
                        calc(&p1, &p2, 8, 3),
                        calc(&p1, &p2, 9, 1),
                    ]
                    .into_iter()
                    .fold((0, 0), |(x1, y1), (x2, y2)| (x1 + x2, y1 + y2));

                    games[index(&GameState(p1, p2))] = total;
                }
            }
        }

        for score2 in 0..=score1 {
            for pos1 in 0..10 {
                for pos2 in 0..10 {
                    let p1 = PlayerState {
                        pos: pos1,
                        score: score2,
                    };
                    let p2 = PlayerState {
                        pos: pos2,
                        score: score1,
                    };

                    let calc = |p1: &PlayerState, p2: &PlayerState, roll, mul| -> (u64, u64) {
                        let mut p1 = p1.clone();
                        p1.turn(roll);
                        let (a, b) = outcome(&GameState(p2.clone(), p1), &games);
                        assert!((a, b) != (0, 0));
                        (b * mul, a * mul)
                    };

                    let total = [
                        calc(&p1, &p2, 3, 1),
                        calc(&p1, &p2, 4, 3),
                        calc(&p1, &p2, 5, 6),
                        calc(&p1, &p2, 6, 7),
                        calc(&p1, &p2, 7, 6),
                        calc(&p1, &p2, 8, 3),
                        calc(&p1, &p2, 9, 1),
                    ]
                    .into_iter()
                    .fold((0, 0), |(x1, y1), (x2, y2)| (x1 + x2, y1 + y2));

                    games[index(&GameState(p1, p2))] = total;
                }
            }
        }
    }

    let (a, b) = outcome(&game, &games);
    let max = if a > b { a } else { b };
    Ok(max)
}

fn outcome(game: &GameState, known: &[(u64, u64)]) -> (u64, u64) {
    if game.1.score >= 21 {
        (0, 1)
    } else {
        known[index(game)]
    }
}

fn index(game: &GameState) -> usize {
    game.0.pos as usize
        + 10 * game.1.pos as usize
        + 100 * game.0.score as usize
        + 2100 * game.1.score as usize
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct PlayerState {
    pos: u8,
    score: u32,
}

impl PlayerState {
    fn turn(&mut self, roll: u8) {
        self.pos = (self.pos + roll) % 10;
        self.score += u32::from(match self.pos {
            0 => 10,
            _ => self.pos,
        });
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct GameState(PlayerState, PlayerState);

mod parse {
    use nom::{
        bytes::complete::tag,
        character::complete::{digit1, line_ending},
        sequence::{separated_pair, tuple},
        IResult,
    };

    use super::*;

    fn player_state(s: &str) -> IResult<&str, PlayerState> {
        let (s, _) = tuple((tag("Player "), digit1, tag(" starting position: ")))(s)?;
        let (s, pos) = nom::character::complete::u8(s)?;
        Ok((s, PlayerState { pos, score: 0 }))
    }

    pub(super) fn input(s: &str) -> IResult<&str, GameState> {
        let (s, (p1, p2)) = separated_pair(player_state, line_ending, player_state)(s)?;

        Ok((s, GameState(p1, p2)))
    }
}
