use core::fmt::Debug;
use std::{fs, num::ParseIntError, str::FromStr};

struct Monkey {
    items: Vec<i128>,
    op: (char, Result<i128, ParseIntError>),
    test: i128,
    next_1: usize,
    next_2: usize,
}

fn read_last_number_of_next_line<'a, I, T>(data: &mut I) -> T
where
    I: Iterator<Item = &'a String>,
    T: std::str::FromStr,
    <T as FromStr>::Err: Debug,
{
    data.next()
        .unwrap()
        .split(" ")
        .last()
        .unwrap()
        .parse()
        .unwrap()
}

fn parse_monkeys(input: Vec<String>) -> Vec<Monkey> {
    let mut monkeys: Vec<Monkey> = Vec::new();

    let mut data = input.iter();
    for _ in 0..8 {
        data.next();

        let items: Vec<i128> = data.next().unwrap()["  Starting items: ".len()..]
            .split(", ")
            .map(|num| num.parse().unwrap())
            .collect();

        let op = data.next().unwrap()["  Operation: new = old ".len()..].split_at(1);
        let op: (char, Result<i128, _>) = (op.0.as_bytes()[0] as char, op.1[1..].parse());

        let test: i128 = read_last_number_of_next_line(&mut data);

        let next_1: usize = read_last_number_of_next_line(&mut data);
        let next_2: usize = read_last_number_of_next_line(&mut data);

        monkeys.push(Monkey {
            items: items,
            op: op,
            test: test,
            next_1: next_1,
            next_2: next_2,
        });

        data.next();
    }

    monkeys
}

fn solve_1(input: Vec<String>) {
    let mut monkeys = parse_monkeys(input);

    let mut items_processed = vec![0; 8];

    for _ in 0..20 {
        for i in 0..8 {
            items_processed[i] += monkeys[i].items.len();
            let items_to_process = monkeys[i].items.clone();
            monkeys[i].items.clear();
            for item in items_to_process {
                let new_worry_level = match monkeys[i].op {
                    ('+', Ok(v)) => item + v,
                    ('*', Ok(v)) => item * v,
                    ('*', Err(_)) => item * item,
                    (_, _) => unreachable!(),
                } / 3;

                let next_1 = monkeys[i].next_1;
                let next_2 = monkeys[i].next_2;
                if new_worry_level % monkeys[i].test == 0 {
                    monkeys[next_1].items.push(new_worry_level);
                } else {
                    monkeys[next_2].items.push(new_worry_level);
                }
            }
        }
    }

    items_processed.sort_unstable();
    let ans = items_processed[6] * items_processed[7];
    println!("{}", ans);
}

fn solve_2(input: Vec<String>) {
    let mut monkeys = parse_monkeys(input);

    let mut items_processed = vec![0; 8];

    let p = monkeys
        .iter()
        .map(|m| m.test)
        .fold(1, |acc, next| acc * next);

    for _ in 0..10000 {
        for i in 0..8 {
            items_processed[i] += monkeys[i].items.len();
            let items_to_process = monkeys[i].items.clone();
            monkeys[i].items.clear();
            for item in items_to_process {
                let new_worry_level = match monkeys[i].op {
                    ('+', Ok(v)) => (item + v) % p,
                    ('*', Ok(v)) => (item * v) % p,
                    ('*', Err(_)) => (item * item) % p,
                    (_, _) => unreachable!(),
                };

                let next_1 = monkeys[i].next_1;
                let next_2 = monkeys[i].next_2;
                if new_worry_level % monkeys[i].test == 0 {
                    monkeys[next_1].items.push(new_worry_level);
                } else {
                    monkeys[next_2].items.push(new_worry_level);
                }
            }
        }
    }

    items_processed.sort_unstable();
    let ans = items_processed[6] * items_processed[7];
    println!("{}", ans);
}

fn main() {
    let input: Vec<String> = fs::read_to_string("input.txt")
        .unwrap()
        .split_terminator('\n')
        .map(|x| x.to_string())
        .collect();

    solve_1(input.clone());
    solve_2(input.clone());
}
