extern crate test;
use itertools::Itertools;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub fn solve() -> (i32, i32) {
    let file = File::open("inputs/input02.txt").unwrap();
    let reader = BufReader::new(file);

    const ROCK: &str = "A";
    const PAPER: &str = "B";
    const SCISSORS: &str = "C";

    const ROCK_OR_LOSE: &str = "X";
    const PAPER_OR_DRAW: &str = "Y";
    const SCISSORS_OR_WIN: &str = "Z";

    const LOSE: i32 = 0;
    const DRAW: i32 = 3;
    const WIN: i32 = 6;

    const S_ROCK: i32 = 1;
    const S_PAPER: i32 = 2;
    const S_SCISSORS: i32 = 3;

    reader.lines().fold((0, 0), |(p1, p2), s| {
        let s0 = s.unwrap();
        let mut s1 = s0.split(" ");
        if let Some(next) = s1.next_tuple() {
            match next {
                (ROCK, ROCK_OR_LOSE) => (p1 + S_ROCK + DRAW, p2 + S_SCISSORS + LOSE),
                (ROCK, PAPER_OR_DRAW) => (p1 + S_PAPER + WIN, p2 + S_ROCK + DRAW),
                (ROCK, SCISSORS_OR_WIN) => (p1 + S_SCISSORS + LOSE, p2 + S_PAPER + WIN),

                (PAPER, ROCK_OR_LOSE) => (p1 + S_ROCK + LOSE, p2 + S_ROCK + LOSE),
                (PAPER, PAPER_OR_DRAW) => (p1 + S_PAPER + DRAW, p2 + S_PAPER + DRAW),
                (PAPER, SCISSORS_OR_WIN) => (p1 + S_SCISSORS + WIN, p2 + S_SCISSORS + WIN),

                (SCISSORS, ROCK_OR_LOSE) => (p1 + S_ROCK + WIN, p2 + S_PAPER + LOSE),
                (SCISSORS, PAPER_OR_DRAW) => (p1 + S_PAPER + LOSE, p2 + S_SCISSORS + DRAW),
                (SCISSORS, SCISSORS_OR_WIN) => (p1 + S_SCISSORS + DRAW, p2 + S_ROCK + WIN),

                _ => panic!(),
            }
        } else {
            (p1, p2)
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solution() {
        assert_eq!((14297, 10498), solve());
    }
}
