// day 20

const DECRYPT_KEY: i64 = 811589153;

fn mix(numbers: &Vec<(usize, i64)>, ring: &mut Vec<(usize, i64)>) {
    for n in numbers {
        let index = ring.iter().position(|r| r.0 == n.0).unwrap();
        let removed = ring.remove(index);
        let len = ring.len() as i64;
        let pos = (index as i64 + removed.1) % len;
        let insert_at = if pos < 0 { len + pos } else { pos } as usize;
        ring.insert(insert_at, removed);
    }
}

fn mix_n(list: &Vec<i64>, n: i32, decrypt_key: i64) -> i64 {
    let numbers: Vec<(usize, i64)> = list
        .iter()
        .enumerate()
        .map(|(i, n)| (i, *n * decrypt_key))
        .collect();

    let mut ring = numbers.clone();

    for _ in 0..n {
        mix(&numbers, &mut ring);
    }

    let len = ring.len();
    let i = ring.iter().position(|&r| r.1 == 0).unwrap();
    let a = ring[(i + 1000) % len];
    let b = ring[(i + 2000) % len];
    let c = ring[(i + 3000) % len];
    a.1 + b.1 + c.1
}

pub fn solve() -> (i64, i64) {
    let bytes = include_bytes!("../inputs/input20.txt");
    let numbers: Vec<i64> = String::from_utf8_lossy(bytes)
        .trim()
        .split("\n")
        .map(|s| s.parse::<i64>().unwrap())
        .collect();

    let p1 = mix_n(&numbers, 1, 1);
    let p2 = mix_n(&numbers, 10, DECRYPT_KEY);
    (p1, p2)
}
