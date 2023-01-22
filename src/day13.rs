use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;
use std::cmp::Ordering;
use std::collections::VecDeque;

#[derive(Debug, Eq)]
enum Packet {
    LIST(Vec<Packet>),
    INT(i64),
}

impl PartialEq for Packet {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::LIST(l0), Self::LIST(r0)) => l0 == r0,
            (Self::INT(l0), Self::INT(r0)) => l0 == r0,
            _ => false,
        }
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (Packet::INT(left), Packet::INT(right)) => left.partial_cmp(right),
            (Packet::LIST(left), Packet::LIST(right)) => left.partial_cmp(right),
            (Packet::INT(num), right) => {
                let wrapped_int = Packet::LIST(vec![Packet::INT(*num)]);
                wrapped_int.partial_cmp(right)
            }
            (left, Packet::INT(num)) => {
                let wrapped_int = Packet::LIST(vec![Packet::INT(*num)]);
                left.partial_cmp(&wrapped_int)
            }
        }
    }
}

#[derive(Debug)]
enum Token {
    LPAREN,
    RPAREN,
    COMMA,
    INT(i64),
}

fn tokenize(input: &str) -> VecDeque<Token> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"(,|\[|\]|\d+)").unwrap();
    }

    RE.find_iter(input)
        .map(|m| match m.as_str() {
            "[" => Token::LPAREN,
            "]" => Token::RPAREN,
            "," => Token::COMMA,
            num => Token::INT(num.parse().unwrap()),
        })
        .collect()
}

fn parse(tokens: &mut VecDeque<Token>, packet: &mut Packet) {
    if let Packet::LIST(list) = packet {
        loop {
            if let Some(token) = tokens.pop_front() {
                match token {
                    Token::LPAREN => {
                        let mut sublist = Packet::LIST(vec![]);
                        parse(tokens, &mut sublist);
                        list.push(sublist);
                        continue;
                    }
                    Token::RPAREN => {
                        return;
                    }
                    Token::COMMA => {
                        continue;
                    }
                    Token::INT(num) => list.push(Packet::INT(num)),
                }
            } else {
                return;
            }
        }
    } else {
        panic!();
    }
}

fn tokenize_and_parse(line: &str) -> Packet {
    let mut tokens = tokenize(line);
    let mut root = Packet::LIST(vec![]);
    parse(&mut tokens, &mut root);
    return root;
}

pub fn solve() -> (i64, i64) {
    let buf = include_bytes!("../inputs/input13.txt");
    let mut p1 = 0;
    let mut index = 1;

    let divider1 = tokenize_and_parse("[[2]]");
    let divider2 = tokenize_and_parse("[[6]]");
    let mut smaller_than_1 = 0;
    let mut between_1_and_2 = 0;

    for (a, b, _) in String::from_utf8_lossy(buf).split("\n").tuples() {
        let packet_a = tokenize_and_parse(a);
        let packet_b = tokenize_and_parse(b);

        if packet_a <= packet_b {
            p1 += index;
        }

        // For part 2, we don't need to sort everything, just keep track of
        // how many packets are before/after the divider packets.
        if packet_a < divider1 {
            smaller_than_1 += 1;
        } else if packet_a < divider2 {
            between_1_and_2 += 1;
        }

        if packet_b < divider1 {
            smaller_than_1 += 1;
        } else if packet_b < divider2 {
            between_1_and_2 += 1;
        }

        index += 1;
    }

    let divider_index1 = smaller_than_1 + 1;
    let divider_index2 = divider_index1 + between_1_and_2 + 1;
    let p2 = divider_index1 * divider_index2;
    (p1, p2)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_parser() {
        let mut tokens = super::tokenize("[1,2,[3,4]]");
        println!("tokens: {:?}", tokens);
        let mut root = super::Packet::LIST(vec![]);
        super::parse(&mut tokens, &mut root);
        println!("parse: {:?}", root);
    }
}
