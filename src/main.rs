#![feature(test)]

mod day01;
mod day02;
mod day03;

fn run_puzzle<F>(name: &str, f: F)
where
    F: Fn(),
{
    const REPS: i32 = 1000;
    let start = std::time::Instant::now();
    for _ in 0..REPS {
        f();
    }
    let avg = start.elapsed().as_micros() / REPS as u128;
    println!("{}: {:?} μs", name, avg);
}

fn main() {
    run_puzzle("day01", || {
        day01::solve();
    });
    run_puzzle("day02", || {
        day02::solve();
    });
    run_puzzle("day03", || {
        day03::solve();
    });
}
