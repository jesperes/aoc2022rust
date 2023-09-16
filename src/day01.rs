pub fn solve() -> (i32, i32) {
    let bytes = include_bytes!("../inputs/input01.txt");
    let s = String::from_utf8_lossy(bytes);

    let mut a: i32 = 0;
    let mut b: i32 = 0;
    let mut c: i32 = 0;

    for group in s.split("\n\n") {
        let sum: i32 = group
            .trim()
            .split("\n")
            .map(|s| s.parse::<i32>().unwrap())
            .sum();

        if sum > a {
            c = b;
            b = a;
            a = sum;
        } else if sum > b {
            c = b;
            b = sum;
        } else if sum > c {
            c = sum;
        }
    }

    return (a, a + b + c);
}
