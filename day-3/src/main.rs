use std::{collections::HashSet, fs};

fn priority_from_item_type(item_type: u8) -> i32 {
    (match item_type as char {
        'a'..='z' => item_type - ('a' as u8) + 1,
        _ => 26 + item_type - ('A' as u8) + 1,
    }) as i32
}

fn solve_1(input: Vec<String>) {
    let ans: i32 = input
        .iter()
        .map(|line| {
            let middle = line.len() / 2;
            let s1 = &line[..middle].bytes().collect::<HashSet<_>>();
            let s2 = &line[middle..].bytes().collect::<HashSet<_>>();
            let inter = *s1.intersection(s2).next().unwrap();
            priority_from_item_type(inter)
        })
        .sum();
    println!("{ans}")
}

fn solve_2(input: Vec<String>) {
    let ans: i32 = input
        .iter()
        .map(|line| line.bytes().collect::<HashSet<_>>())
        .collect::<Vec<_>>()
        .chunks(3)
        .map(|x| (&x[0], &x[1], &x[2]))
        .map(|(s1, s2, s3)| {
            let inter = *s1
                .intersection(s2)
                .map(|c| *c)
                .collect::<HashSet<_>>()
                .intersection(s3)
                .next()
                .unwrap();
            priority_from_item_type(inter)
        })
        .sum();
    println!("{ans}")
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
