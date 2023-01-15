pub fn solve() -> (i32, i32) {
    let buf = include_bytes!("../inputs/input06.txt");
    let mut p1: i32 = 0;
    let mut p2: i32 = 0;

    for i in 0..buf.len() {
        let a = buf[i];
        let b = buf[i + 1];
        let c = buf[i + 2];
        let d = buf[i + 3];

        if a == b || a == c || a == d || b == c || b == d || c == d {
            continue;
        } else {
            p1 = i as i32;
            break;
        }
    }

    // well...
    for i in p1 as usize..buf.len() - 14 {
        let a = buf[i];
        let b = buf[i + 1];
        let c = buf[i + 2];
        let d = buf[i + 3];
        let e = buf[i + 4];
        let f = buf[i + 5];
        let g = buf[i + 6];
        let h = buf[i + 7];
        let j = buf[i + 8];
        let k = buf[i + 9];
        let l = buf[i + 10];
        let m = buf[i + 11];
        let n = buf[i + 12];
        let o = buf[i + 13];

        if a == b
            || a == c
            || a == d
            || a == e
            || a == f
            || a == g
            || a == h
            || a == j
            || a == k
            || a == l
            || a == m
            || a == n
            || a == o

            // b
            || b == c
            || b == d
            || b == e
            || b == f
            || b == g
            || b == h
            || b == j
            || b == k
            || b == l
            || b == m
            || b == n
            || b == o

            // c
            || c == d
            || c == e
            || c == f
            || c == g
            || c == h
            || c == j
            || c == k
            || c == l
            || c == m
            || c == n
            || c == o

            // d
            || d == e
            || d == f
            || d == g
            || d == h
            || d == j
            || d == k
            || d == l
            || d == m
            || d == n
            || d == o

            // d
            || e == f
            || e == g
            || e == h
            || e == j
            || e == k
            || e == l
            || e == m
            || e == n
            || e == o

            // d
            || f == g
            || f == h
            || f == j
            || f == k
            || f == l
            || f == m
            || f == n
            || f == o

            // g
            || g == h
            || g == j
            || g == k
            || g == l
            || g == m
            || g == n
            || g == o

            // h
            || h == j
            || h == k
            || h == l
            || h == m
            || h == n
            || h == o

            // j
            || j == k
            || j == l
            || j == m
            || j == n
            || j == o

            // k
            || k == l
            || k == m
            || k == n
            || k == o

            // l
            || l == m
            || l == n
            || l == o

            // m
            || m == n
            || m == o

            // n
            || n == o
        {
            continue;
        } else {
            p2 = i as i32;
            break;
        }
    }

    p1 += 4;
    p2 += 14;
    (p1, p2)
}
