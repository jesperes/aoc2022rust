pub fn solve() -> String {
    rev_snafu(
        String::from_utf8_lossy(include_bytes!("../inputs/input25.txt"))
            .split("\n")
            .fold(0i64, |sum, line| snafu(line) + sum),
    )
}

fn snafu(s: &str) -> i64 {
    let (_, sum) = s.chars().rev().fold((1i64, 0), |(n, sum), c| {
        let m = match c {
            '2' => sum + 2 * n,
            '1' => sum + 1 * n,
            '0' => sum,
            '-' => sum - 1 * n,
            '=' => sum - 2 * n,
            _ => unreachable!(),
        };
        (n * 5, m)
    });
    sum
}

fn rev_snafu(n: i64) -> String {
    let mut s: Vec<char> = Vec::new();
    let mut n0 = n;
    loop {
        if n0 == 0 {
            return s.iter().rev().collect::<String>();
        }

        let index = (n0 + 2) % 5;
        let c = snafu_char(index);
        if index < 2 {
            n0 = (n0 + 5).div_floor(5);
        } else {
            n0 = n0.div_floor(5);
        }

        s.push(c);
    }
}

fn snafu_char(n: i64) -> char {
    match n {
        0 => '=',
        1 => '-',
        2 => '0',
        3 => '1',
        4 => '2',
        _ => unreachable!(),
    }
}
