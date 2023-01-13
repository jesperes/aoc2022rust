#![feature(test)]

mod day01;
mod day02;
mod day03;
mod day04;

fn run_puzzle<F>(name: &str, f: F)
where
    F: Fn(),
{
    const MAX_REPS: u128 = 10_000;
    const MAX_SECS: u64 = 1;
    let start = std::time::Instant::now();
    let mut counter: u128 = 0;
    loop {
        f();
        counter += 1;
        if start.elapsed().as_secs() > MAX_SECS || counter >= MAX_REPS {
            break;
        }
    }
    let avg = start.elapsed().as_nanos() / counter;
    println!("{}: {:?} ns ({} iters)", name, avg, counter);
}

fn main() {
    run_puzzle("day01", || {
        assert_eq!((69836, 207968), day01::solve());
    });
    run_puzzle("day02", || {
        assert_eq!((14297, 10498), day02::solve());
    });
    run_puzzle("day03", || {
        assert_eq!((8349, 2681), day03::solve());
    });
    run_puzzle("day04", || {
        assert_eq!((582, 893), day04::solve());
    });
}
