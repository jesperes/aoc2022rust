enum Op {
    PLUS(i64),
    MULT(i64),
    SQUARED,
    UNDEFINED,
}

fn eval(op: &Op, arg: i64) -> i64 {
    match op {
        Op::MULT(v) => v * arg,
        Op::PLUS(v) => v + arg,
        Op::SQUARED => arg * arg,
        _ => panic!(),
    }
}

struct Monkey {
    num: usize,
    items: Vec<i64>,
    op: Op,
    divisible_by: i64,
    on_true: i64,
    on_false: i64,
}

fn parse(buf: &[u8]) -> Vec<Monkey> {
    let mut monkeys: Vec<Monkey> = Vec::new();
    let mut current: usize = 0;

    for line in String::from_utf8_lossy(buf).trim().split("\n") {
        let words = line.trim().split([' ', ':', ',']);
        let words_vec = words.collect::<Vec<&str>>();
        // println!("words = {:?}", words_vec);
        match words_vec[..] {
            ["Monkey", ..] => {
                current = monkeys.len();
                monkeys.push(Monkey {
                    num: current,
                    items: Vec::new(),
                    op: Op::UNDEFINED,
                    on_true: 0,
                    divisible_by: 0,
                    on_false: 0,
                });
            }
            ["Starting", "items", ..] => {
                let monkey = &mut monkeys[current];

                for s in &words_vec[3..] {
                    if let Ok(n) = s.parse::<i64>() {
                        monkey.items.push(n);
                    }
                }
            }
            ["Operation", .., "old", "*", "old"] => {
                monkeys[current].op = Op::SQUARED;
            }
            ["Operation", .., "old", "*", num] => {
                monkeys[current].op = Op::MULT(num.parse().unwrap());
            }
            ["Operation", .., "old", "+", num] => {
                monkeys[current].op = Op::PLUS(num.parse().unwrap())
            }
            ["Test", _, "divisible", "by", num] => {
                monkeys[current].divisible_by = num.parse().unwrap();
            }
            ["If", "true", .., num] => {
                monkeys[current].on_true = num.parse().unwrap();
            }
            ["If", "false", .., num] => {
                monkeys[current].on_false = num.parse().unwrap();
            }
            [""] => {}
            _ => panic!(),
        }
    }
    monkeys
}

fn simulate(monkeys: &Vec<Monkey>, rounds: i32, part1: bool) -> i64 {
    let mut count: Vec<i64> = Vec::new();
    let mut items: Vec<Vec<i64>> = Vec::new();
    let mut lcd: i64 = 1;

    for i in 0..monkeys.len() {
        count.push(0);
        items.push(monkeys[i].items.clone());
        if !part1 {
            lcd *= monkeys[i].divisible_by;
        }
    }

    for _ in 1..=rounds {
        for monkey in monkeys {
            let monkey_idx = monkey.num;
            count[monkey_idx] += items[monkey_idx].len() as i64;
            let items_to_move: Vec<i64> = items[monkey_idx].clone();
            items[monkey_idx].clear();

            for item in items_to_move {
                let x = eval(&monkey.op, item);
                let worry_level: i64 = match part1 {
                    true => x / 3,
                    false => x % lcd,
                };
                let dest_monkey = if worry_level % monkey.divisible_by == 0 {
                    monkey.on_true
                } else {
                    monkey.on_false
                };
                items[dest_monkey as usize].push(worry_level);
            }
            items[monkey_idx].clear();
        }
    }

    let mut a: usize = 0;
    let mut b: usize = 0;

    for i in 0..count.len() {
        let c = count[i] as usize;
        if c > a {
            b = a;
            a = c
        } else if c > b {
            b = c;
        }
    }

    (a * b) as i64
}

pub fn solve() -> (i64, i64) {
    let buf = include_bytes!("../inputs/input11.txt");
    let monkeys = parse(buf);
    let p1 = simulate(&monkeys, 20, true);
    let p2 = simulate(&monkeys, 10000, false);
    (p1, p2)
}
