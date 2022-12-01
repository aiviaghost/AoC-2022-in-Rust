use std::fs;

fn solve_1(input: Vec<String>) {
    let ans: i32 = input
        .iter()
        .map(|x| {
            x.split_terminator("\n")
                .map(|i| i.parse::<i32>().unwrap())
                .sum()
        })
        .max()
        .unwrap();

    println!("{ans}");
}

fn solve_2(input: Vec<String>) {
    let mut xs = input
        .iter()
        .map(|x| {
            x.split_terminator("\n")
                .map(|i| i.parse::<i32>().unwrap())
                .sum()
        })
        .collect::<Vec<i32>>();
    xs.sort();
    let ans: i32 = xs.iter().rev().take(3).sum();
    println!("{ans}");
}

fn main() {
    let input: Vec<String> = fs::read_to_string("input.txt")
        .unwrap()
        .split_terminator("\n\n")
        .map(|x| x.to_string())
        .collect();

    solve_1(input.clone());
    solve_2(input.clone());
}
