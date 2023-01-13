use std::ops::RangeInclusive;

fn to_int(opt: Option<&str>) -> i32 {
    opt.unwrap().parse::<i32>().unwrap()
}

pub fn solve() -> (u32, u32) {
    let buf = include_bytes!("../inputs/input04.txt");
    let s = String::from_utf8_lossy(buf);
    let mut p1: u32 = 0;
    let mut p2: u32 = 0;

    for line in s.trim().split("\n") {
        let mut items = line.split([',', '-']);
        let a = to_int(items.next());
        let b = to_int(items.next());
        let c = to_int(items.next());
        let d = to_int(items.next());
        let a_range = a..=b;
        let b_range = c..=d;
        if contains_either(&a_range, &b_range) {
            p1 += 1;
        }
        if overlaps(&a_range, &b_range) {
            p2 += 1;
        }
    }
    (p1, p2)
}

fn contains_either(a: &RangeInclusive<i32>, b: &RangeInclusive<i32>) -> bool {
    (a.contains(b.start()) && a.contains(b.end())) || (b.contains(a.start()) && b.contains(a.end()))
}

fn overlaps(a: &RangeInclusive<i32>, b: &RangeInclusive<i32>) -> bool {
    (a.end() >= b.start() && a.end() <= b.end()) || (b.end() >= a.start()) && (b.end() <= a.end())
}
