type Crt = [char; 240]; // 40 * 6 chars

pub fn solve() -> (i64, String) {
    let buf = include_bytes!("../inputs/input10.txt");

    let p1 = solve_p1(buf);
    let p2 = solve_p2(buf);

    (p1, p2)
}

fn solve_p1(buf: &[u8]) -> i64 {
    let mut sum: i64 = 0;
    let mut x: i64 = 1;
    let mut cycle: i64 = 1;

    for line in String::from_utf8_lossy(buf).trim().lines() {
        match line.split(' ').collect::<Vec<&str>>()[..] {
            ["noop"] if is_interesting_cycle(cycle) => {
                sum += cycle * x;
                cycle += 1;
            }
            ["noop"] => {
                cycle += 1;
            }
            ["addx", num] if is_interesting_cycle(cycle) => {
                let n = num.parse::<i64>().unwrap();
                sum += cycle * x;
                x += n;
                cycle += 2;
            }
            ["addx", num] if is_interesting_cycle(cycle + 1) => {
                let n = num.parse::<i64>().unwrap();
                sum += (cycle + 1) * x;
                x += n;
                cycle += 2;
            }
            ["addx", num] => {
                let n = num.parse::<i64>().unwrap();
                x += n;
                cycle += 2;
            }
            _ => panic!(),
        };
    }

    sum
}

fn solve_p2(buf: &[u8]) -> String {
    let mut x: usize = 1;
    let mut pos: usize = 0;
    let mut crt: Crt = [' '; 240];

    for line in String::from_utf8_lossy(buf).trim().lines() {
        match line.split(' ').collect::<Vec<&str>>()[..] {
            ["noop"] => {
                draw(x, pos, &mut crt);
                pos += 1;
            }
            ["addx", num] => {
                let n = num.parse::<i32>().unwrap();
                draw(x, pos, &mut crt);
                draw(x, pos + 1, &mut crt);
                pos += 2;
                x = ((x as i32) + n) as usize;
            }
            _ => panic!(),
        };
    }

    crt.iter().collect()
}

fn is_interesting_cycle(cycle: i64) -> bool {
    (cycle - 20) % 40 == 0
}

fn draw(x: usize, pos: usize, crt: &mut Crt) {
    let x0 = x as i32;
    let pos40 = (pos % 40) as i32;
    if pos40 == x0 - 1 || pos40 == x0 || pos40 == x0 + 1 {
        crt[pos] = '█';
    } else {
        crt[pos] = '.';
    }
}
