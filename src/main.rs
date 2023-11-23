#![feature(test, int_roundings)]

use clap::Parser;
use regex::Regex;
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
mod day18;
mod day19;
mod day20;
mod day21;

#[derive(Parser)]
struct Cli {
    puzzles: Vec<String>,
}

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

fn run_puzzles(puzzles: Vec<Puzzle>, max_reps: usize) {
    for p in puzzles {
        const MAX_SECS: u64 = 5;
        let mut runtimes: Vec<Duration> = vec![];
        let start = std::time::Instant::now();

        loop {
            let t = std::time::Instant::now();
            (p.fun)();
            runtimes.push(t.elapsed());
            if start.elapsed().as_secs() >= MAX_SECS || runtimes.len() >= max_reps {
                break;
            }
        }

        let avg_runtime = runtimes.iter().sum::<Duration>() / runtimes.len() as u32;
        println!(
            "Day {}: {:10} μs {:10} ns ({} reps)",
            p.name,
            avg_runtime.as_micros(),
            avg_runtime.as_nanos(),
            runtimes.len()
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

fn matches(cli: &Cli, name: &String) -> bool {
    for p in &cli.puzzles {
        if let Ok(re) = Regex::new(name) {
            if re.is_match(p) {
                return true;
            }
        }
    }
    false
}

fn main() {
    let args = Cli::parse();
    let all_puzzles = vec![
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
        Puzzle::make(18, || assert_eq!((3530, 2000), day18::solve())),
        Puzzle::make(19, || assert_eq!((1382, 31740), day19::solve())),
        Puzzle::make(20, || assert_eq!((7278, 14375678667089), day20::solve())),
        Puzzle::make(21, || {
            assert_eq!((268597611536314, 3451534022348), day21::solve())
        }),
    ];
    if args.puzzles.len() == 0 {
        println!("Running all puzzles.");
        run_puzzles(all_puzzles, 100);
    } else {
        // Only run specified puzzles
        let subset: Vec<Puzzle> = all_puzzles
            .into_iter()
            .filter(|p| matches(&args, &p.name))
            .collect();

        if subset.len() == 0 {
            println!(
                "Puzzles specified do not match any implementations: {:?}",
                args.puzzles
            );
        } else {
            run_puzzles(subset, 1);
        }
    }
}
