extern crate test;
use itertools::Itertools;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub fn solve() -> (i32, i32) {
    let file = File::open("inputs/input02.txt").unwrap();
    let reader = BufReader::new(file);
    reader.lines().fold((0, 0), |(p1, p2), s| {
        let s0 = s.unwrap();
        let mut s1 = s0.split(" ");
        let next: Option<(&str, &str)> = s1.next_tuple();
        match next {
            Some(("A", "X")) => (p1 + 1, p2 + 1),
            Some((_, _)) => (p1 + 2, p2 + 3),
            None => (p1, p2),
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn solution() {
        assert_eq!((69836, 207968), solve());
    }

    #[bench]
    fn bench(b: &mut Bencher) {
        b.iter(|| solve());
    }
}
