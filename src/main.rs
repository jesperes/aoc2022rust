#![feature(test, int_roundings)]

use std::time::Duration;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
struct Puzzle {
    name: String,
    fun: fn(),
}

impl Puzzle {
    fn make(day: i32, fun: fn()) -> Puzzle {
        Puzzle {
            name: format!("{:02}", day),
            fun,
        }
    }
}

fn run_puzzles(puzzles: Vec<Puzzle>) {
    for p in puzzles {
        if p.name != "17" {
            continue;
        }

        const MAX_REPS: usize = 1;
        const MAX_SECS: u64 = 1;
        let mut runtimes: Vec<Duration> = vec![];
        let start = std::time::Instant::now();

        loop {
            let t = std::time::Instant::now();
            (p.fun)();
            runtimes.push(t.elapsed());
            if start.elapsed().as_secs() >= MAX_SECS || runtimes.len() >= MAX_REPS {
                break;
            }
        }

        let avg_runtime = runtimes.iter().sum::<Duration>() / runtimes.len() as u32;
        println!(
            "Day {}: {:10} μs {:10} ns",
            p.name,
            avg_runtime.as_micros(),
            avg_runtime.as_nanos()
        );
    }
}

fn day5_sol() -> (String, String) {
    ("CNSZFDVLJ".to_string(), "QNDWLMGNS".to_string())
}

fn day10_sol() -> (i64, String) {
    (
        14060,
        "\
███...██..███..█..█.████.█..█.████...██.\
█..█.█..█.█..█.█.█..█....█.█..█.......█.\
█..█.█..█.█..█.██...███..██...███.....█.\
███..████.███..█.█..█....█.█..█.......█.\
█....█..█.█....█.█..█....█.█..█....█..█.\
█....█..█.█....█..█.█....█..█.████..██..\
"
        .to_string(),
    )
}

fn main() {
    run_puzzles(vec![
        Puzzle::make(1, || assert_eq!((69836, 207968), day01::solve())),
        Puzzle::make(2, || assert_eq!((14297, 10498), day02::solve())),
        Puzzle::make(3, || assert_eq!((8349, 2681), day03::solve())),
        Puzzle::make(4, || assert_eq!((582, 893), day04::solve())),
        Puzzle::make(5, || assert_eq!(day5_sol(), day05::solve())),
        Puzzle::make(6, || assert_eq!((1802, 3551), day06::solve())),
        Puzzle::make(7, || assert_eq!((1543140, 1117448), day07::solve())),
        Puzzle::make(8, || assert_eq!((1684, 486540), day08::solve())),
        Puzzle::make(9, || assert_eq!((6311, 2482), day09::solve())),
        Puzzle::make(10, || assert_eq!(day10_sol(), day10::solve())),
        Puzzle::make(11, || assert_eq!((102399, 23641658401), day11::solve())),
        Puzzle::make(12, || assert_eq!((370, 363), day12::solve())),
        Puzzle::make(13, || assert_eq!((5198, 22344), day13::solve())),
        Puzzle::make(14, || assert_eq!((696, 23610), day14::solve())),
        Puzzle::make(15, || assert_eq!((4665948, 13543690671045), day15::solve())),
        Puzzle::make(16, || assert_eq!((1376, 1933), day16::solve())),
        Puzzle::make(17, || assert_eq!((3153, 1553665689155), day17::solve())),
    ]);
}
