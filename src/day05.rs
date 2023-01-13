const NUM_STACKS: usize = 9;
type Stack = Vec<char>;
type Stacks = Vec<Stack>;

pub fn solve() -> (String, String) {
    let buf = include_bytes!("../inputs/input05.txt");
    let s = String::from_utf8_lossy(buf);
    let mut stacks1: Stacks = Vec::new();
    let mut stacks2: Stacks = Vec::new();

    for _i in 0..NUM_STACKS {
        stacks1.push(Vec::new());
        stacks2.push(Vec::new());
    }

    let mut parts = s.split("\n\n");
    let stack_part = parts.next().unwrap();
    let command_part = parts.next().unwrap();

    for line in stack_part.split("\n") {
        let bytes = line.as_bytes();
        parse_crate_line(bytes, &mut stacks1);
        parse_crate_line(bytes, &mut stacks2);
    }

    for line in command_part.trim().split("\n") {
        let mut words = line.split(" ");
        words.next();
        let n = words.next().unwrap().parse::<usize>().unwrap();
        words.next();
        let from_idx = words.next().unwrap().parse::<usize>().unwrap() - 1;
        words.next();
        let to_idx = words.next().unwrap().parse::<usize>().unwrap() - 1;

        let mut to_move1: Stack = stacks1[from_idx].drain(0..n).collect();
        to_move1.reverse();
        stacks1[to_idx].splice(0..0, to_move1);

        let to_move2: Stack = stacks2[from_idx].drain(0..n).collect();
        stacks2[to_idx].splice(0..0, to_move2);
    }

    (top_crates(&stacks1), top_crates(&stacks2))
}

fn top_crates(stacks: &Stacks) -> String {
    stacks
        .iter()
        .map(|stack| stack.first().unwrap())
        .collect::<String>()
}

fn parse_crate_line(line: &[u8], stacks: &mut Stacks) {
    for i in 0..NUM_STACKS {
        let c: u8 = line[i * 4 + 1];
        if c == 32 {
            continue;
        } else {
            stacks[i].push(c as char);
        }
    }
}
