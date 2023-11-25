use lazy_regex::regex_captures;
use std::collections::HashMap;

#[derive(Debug)]
enum Rule {
    MathOp(String, char, String),
    Number(i64),
}

#[derive(Debug)]
enum Expr {
    Humn,
    Num(i64),
    Expr(Box<Expr>, char, Box<Expr>),
}

impl Expr {
    fn make(lhs: Expr, op: char, rhs: Expr) -> Expr {
        Expr::Expr(Box::new(lhs), op, Box::new(rhs))
    }
}

fn yell(monkey: &str, rules: &HashMap<String, Rule>) -> i64 {
    match rules.get(monkey).unwrap() {
        Rule::MathOp(lhs, op, rhs) => match op {
            '+' => yell(lhs, rules) + yell(rhs, rules),
            '-' => yell(lhs, rules) - yell(rhs, rules),
            '*' => yell(lhs, rules) * yell(rhs, rules),
            '/' => yell(lhs, rules) / yell(rhs, rules),
            _ => unreachable!(),
        },
        Rule::Number(num) => *num,
    }
}

fn solve_for_humn(monkey: &str, rules: &HashMap<String, Rule>) -> Expr {
    if monkey == "humn" {
        return Expr::Humn;
    }
    match rules.get(monkey).unwrap() {
        Rule::Number(num) => Expr::Num(*num),
        Rule::MathOp(lhs, '+', rhs) if monkey == "root" => reduce(
            eval(solve_for_humn(lhs, rules)),
            eval(solve_for_humn(rhs, rules)),
        ),
        Rule::MathOp(lhs, op, rhs) => {
            Expr::make(solve_for_humn(lhs, rules), *op, solve_for_humn(rhs, rules))
        }
    }
}

fn eval(expr: Expr) -> Expr {
    match expr {
        Expr::Num(_) => expr,
        Expr::Humn => expr,
        Expr::Expr(lhs, op, rhs) => {
            let eval_lhs = eval(*lhs);
            let eval_rhs = eval(*rhs);
            match (eval_lhs, eval_rhs) {
                (Expr::Num(lhs_num), Expr::Num(rhs_num)) => match op {
                    '+' => Expr::Num(lhs_num + rhs_num),
                    '-' => Expr::Num(lhs_num - rhs_num),
                    '*' => Expr::Num(lhs_num * rhs_num),
                    '/' => Expr::Num(lhs_num / rhs_num),
                    _ => unreachable!(),
                },
                (l, r) => Expr::make(l, op, r),
            }
        }
    }
}

fn reduce(lhs: Expr, rhs: Expr) -> Expr {
    match (lhs, rhs) {
        (l, r @ Expr::Num(_)) => {
            reduce(r, l) // flip order so that lhs always is a number
        }
        (num_expr, Expr::Humn) => num_expr,
        (Expr::Num(num), Expr::Expr(lhs_box, op, rhs_box)) => {
            let lhs0 = *lhs_box;
            let rhs0 = *rhs_box;

            match (lhs0, op, rhs0) {
                (Expr::Num(l), '+', r) => reduce(Expr::Num(num - l), r),
                (l, '+', Expr::Num(r)) => reduce(l, Expr::Num(num - r)),
                (Expr::Num(l), '-', r) => reduce(Expr::Num(l - num), r),
                (l, '-', Expr::Num(r)) => reduce(l, Expr::Num(num + r)),
                (Expr::Num(l), '*', r) => reduce(Expr::Num(num.div_floor(l)), r),
                (l, '*', Expr::Num(r)) => reduce(Expr::Num(num.div_floor(r)), l),
                (Expr::Num(l), '/', r) => reduce(Expr::Num(num * l), r),
                (l, '/', Expr::Num(r)) => reduce(Expr::Num(num * r), l),
                _ => unreachable!(),
            }
        }
        _ => unreachable!(),
    }
}

pub fn solve() -> (i64, i64) {
    let bytes = include_bytes!("../inputs/input21.txt");
    let rules = String::from_utf8_lossy(bytes)
        .trim()
        .split("\n")
        .map(|line| {
            if let Some((_, m, lhs, op, rhs)) = regex_captures!(r"(.+): (.+) (.*) (.*)", line) {
                (
                    m.to_string(),
                    Rule::MathOp(lhs.to_string(), op.chars().next().unwrap(), rhs.to_string()),
                )
            } else if let Some((_, m, num)) = regex_captures!(r"(.*): (.*)", line) {
                (m.to_string(), Rule::Number(num.parse::<i64>().unwrap()))
            } else {
                unreachable!()
            }
        })
        .collect::<HashMap<String, Rule>>();

    let p1 = yell("root", &rules);
    let p2 = if let Expr::Num(num) = solve_for_humn("root", &rules) {
        num
    } else {
        unreachable!();
    };
    (p1, p2)
}
