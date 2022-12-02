use std::fs;

fn compute_calories_per_elf(input: Vec<String>) -> Vec<i32> {
    input
        .iter()
        .map(|x| {
            x.split_terminator("\n")
                .map(|i| i.parse::<i32>().unwrap())
                .sum()
        })
        .collect()
}

fn solve_1(input: Vec<String>) {
    let ans = *compute_calories_per_elf(input).iter().max().unwrap();

    println!("{ans}");
}

fn solve_2(input: Vec<String>) {
    let mut xs = compute_calories_per_elf(input);
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
