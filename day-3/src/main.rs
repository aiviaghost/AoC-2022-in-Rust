use std::{collections::HashSet, fs};

fn solve_1(input: Vec<String>) {
    let ans: i32 = input
        .iter()
        .map(|line| {
            let middle = line.len() / 2;
            let b1 = &line[0..middle].bytes().collect::<HashSet<_>>();
            let b2 = &line[middle..line.len()].bytes().collect::<HashSet<_>>();
            let inter = b1.intersection(b2).next().unwrap();
            (match *inter as char {
                'a'..='z' => inter - ('a' as u8) + 1,
                _ => 26 + inter - ('A' as u8) + 1,
            }) as i32
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
            (match inter as char {
                'a'..='z' => inter - ('a' as u8) + 1,
                _ => 26 + inter - ('A' as u8) + 1,
            }) as i32
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
