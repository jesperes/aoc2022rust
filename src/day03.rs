extern crate test;
use std::{fs::File, io::Read};

pub fn solve() -> i32 {
    let file = File::open("inputs/input03.txt").unwrap();
    for c in file.bytes() {
        println!("{}", c.unwrap());
    }

    return 0;
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn solution() {
        assert_eq!(0, solve());
    }

    #[bench]
    fn bench(b: &mut Bencher) {
        b.iter(|| solve());
    }
}
