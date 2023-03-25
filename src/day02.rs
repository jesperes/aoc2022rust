const ROCK: u8 = 65; // A
const PAPER: u8 = 66; // B
const SCISSORS: u8 = 67; // C

const ROCK_OR_LOSE: u8 = 88; // X
const PAPER_OR_DRAW: u8 = 89; // Y
const SCISSORS_OR_WIN: u8 = 90; // Z

const LOSE: i32 = 0;
const DRAW: i32 = 3;
const WIN: i32 = 6;

const S_ROCK: i32 = 1;
const S_PAPER: i32 = 2;
const S_SCISSORS: i32 = 3;

pub fn solve() -> (i32, i32) {
    let buf = include_bytes!("../inputs/input02.txt");
    let mut i: usize = 0;
    let mut p1: i32 = 0;
    let mut p2: i32 = 0;

    loop {
        if i >= buf.len() {
            break;
        }
        let left = buf[i];
        let right = buf[i + 2];
        i += 4;

        match (left, right) {
            (ROCK, ROCK_OR_LOSE) => {
                p1 += S_ROCK + DRAW;
                p2 += S_SCISSORS + LOSE;
            }
            (ROCK, PAPER_OR_DRAW) => {
                p1 += S_PAPER + WIN;
                p2 += S_ROCK + DRAW;
            }
            (ROCK, SCISSORS_OR_WIN) => {
                p1 += S_SCISSORS + LOSE;
                p2 += S_PAPER + WIN;
            }
            (PAPER, ROCK_OR_LOSE) => {
                p1 += S_ROCK + LOSE;
                p2 += S_ROCK + LOSE;
            }
            (PAPER, PAPER_OR_DRAW) => {
                p1 += S_PAPER + DRAW;
                p2 += S_PAPER + DRAW;
            }
            (PAPER, SCISSORS_OR_WIN) => {
                p1 += S_SCISSORS + WIN;
                p2 += S_SCISSORS + WIN;
            }
            (SCISSORS, ROCK_OR_LOSE) => {
                p1 += S_ROCK + WIN;
                p2 += S_PAPER + LOSE;
            }
            (SCISSORS, PAPER_OR_DRAW) => {
                p1 += S_PAPER + LOSE;
                p2 += S_SCISSORS + DRAW;
            }
            (SCISSORS, SCISSORS_OR_WIN) => {
                p1 += S_SCISSORS + DRAW;
                p2 += S_ROCK + WIN;
            }
            _ => panic!(),
        }
    }

    return (p1, p2);
}
