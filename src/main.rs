#![feature(test)]

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

type NanoSecs = u128;

struct Puzzle {
    name: String,
    fun: fn(),
}

fn run_puzzles(puzzles: Vec<Puzzle>) {
    let mut total_avg_runtime: NanoSecs = 0;

    for p in puzzles {
        const MAX_REPS: usize = 100;
        const MAX_SECS: u64 = 1;
        let mut runtimes: Vec<NanoSecs> = vec![];
        let start = std::time::Instant::now();

        loop {
            let t = std::time::Instant::now();
            (p.fun)();
            runtimes.push(t.elapsed().as_nanos());
            if start.elapsed().as_secs() >= MAX_SECS || runtimes.len() >= MAX_REPS {
                break;
            }
        }

        let avg: NanoSecs = runtimes.iter().sum::<NanoSecs>() / runtimes.len() as NanoSecs;
        total_avg_runtime += avg;
        println!("{}: {:10} μs", p.name, (avg as f64 / 1_000.0) as i64);
    }

    println!(
        "Total runtime (avg): {} μs",
        (total_avg_runtime as f64 / 1000.0) as i64
    );
}

fn main() {
    run_puzzles(vec![
        Puzzle {
            name: "day01".to_string(),
            fun: || {
                assert_eq!((69836, 207968), day01::solve());
            },
        },
        Puzzle {
            name: "day02".to_string(),
            fun: || {
                assert_eq!((14297, 10498), day02::solve());
            },
        },
        Puzzle {
            name: "day03".to_string(),
            fun: || {
                assert_eq!((8349, 2681), day03::solve());
            },
        },
        Puzzle {
            name: "day04".to_string(),
            fun: || {
                assert_eq!((582, 893), day04::solve());
            },
        },
        Puzzle {
            name: "day05".to_string(),
            fun: || {
                assert_eq!(
                    ("CNSZFDVLJ".to_string(), "QNDWLMGNS".to_string()),
                    day05::solve()
                );
            },
        },
        Puzzle {
            name: "day06".to_string(),
            fun: || {
                assert_eq!((1802, 3551), day06::solve());
            },
        },
        Puzzle {
            name: "day07".to_string(),
            fun: || {
                assert_eq!((1543140, 1117448), day07::solve());
            },
        },
        Puzzle {
            name: "day08".to_string(),
            fun: || {
                assert_eq!((1684, 486540), day08::solve());
            },
        },
        Puzzle {
            name: "day09".to_string(),
            fun: || {
                assert_eq!((6311, 2482), day09::solve());
            },
        },
        Puzzle {
            name: "day10".to_string(),
            fun: || {
                assert_eq!(
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
                        .to_string()
                    ),
                    day10::solve()
                );
            },
        },
        Puzzle {
            name: "day11".to_string(),
            fun: || {
                assert_eq!((102399, 23641658401), day11::solve());
            },
        },
    ]);
}
