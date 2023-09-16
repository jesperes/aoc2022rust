use itertools::Itertools;

type Mask = u64;
type Prio = i32;

pub fn solve() -> (u32, u32) {
    let buf = include_bytes!("../inputs/input03.txt");
    let s = String::from_utf8_lossy(buf);

    let mut p1: Prio = 0;
    let mut p2: Prio = 0;

    for (a, b, c) in s.split("\n").tuples() {
        p1 += count1(a) + count1(b) + count1(c);
        p2 += count2(a, b, c);
    }

    return (p1 as u32, p2 as u32);
}

fn count1(s: &str) -> Prio {
    let (left, right) = s.split_at(s.len() >> 1);
    let left_mask = str_to_mask(left);
    let right_mask = str_to_mask(right);
    mask_to_prio(left_mask & right_mask)
}

fn count2(a: &str, b: &str, c: &str) -> Prio {
    let mask_a = str_to_mask(a);
    let mask_b = str_to_mask(b);
    let mask_c = str_to_mask(c);
    mask_to_prio(mask_a & mask_b & mask_c)
}

fn str_to_mask(s: &str) -> Mask {
    let mut mask: Mask = 0;
    for c in s.chars() {
        mask |= (1 as Mask) << prio(c);
    }
    mask
}

fn mask_to_prio(mask: Mask) -> Prio {
    (mask as f32).log2() as Prio
}

fn prio(item: char) -> u8 {
    let c = item as u8;
    if c <= 90 {
        c - 38
    } else {
        c - 96
    }
}
