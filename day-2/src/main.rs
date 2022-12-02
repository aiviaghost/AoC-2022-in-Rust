use std::fs;

fn get_char_pairs(input: Vec<String>) -> Vec<(char, char)> {
    input
        .iter()
        .map(|line| line.split(" ").map(|x| x.as_bytes()))
        .map(|mut t| {
            let a = t.next().unwrap()[0] as char;
            let b = t.next().unwrap()[0] as char;
            (a, b)
        })
        .collect()
}

fn solve_1(input: Vec<String>) {
    let ans: i32 = get_char_pairs(input)
        .iter()
        .map(|(a, b)| match (a, b) {
            ('A', 'X') => 1 + 3,
            ('A', 'Y') => 2 + 6,
            ('A', 'Z') => 3 + 0,
            ('B', 'X') => 1 + 0,
            ('B', 'Y') => 2 + 3,
            ('B', 'Z') => 3 + 6,
            ('C', 'X') => 1 + 6,
            ('C', 'Y') => 2 + 0,
            ('C', 'Z') => 3 + 3,
            (_, _) => 0,
        })
        .sum();
    println!("{ans}");
}

fn solve_2(input: Vec<String>) {
    let ans: i32 = get_char_pairs(input)
        .iter()
        .map(|(a, b)| match (a, b) {
            ('A', 'X') => 0 + 3,
            ('A', 'Y') => 3 + 1,
            ('A', 'Z') => 6 + 2,
            ('B', 'X') => 0 + 1,
            ('B', 'Y') => 3 + 2,
            ('B', 'Z') => 6 + 3,
            ('C', 'X') => 0 + 2,
            ('C', 'Y') => 3 + 3,
            ('C', 'Z') => 6 + 1,
            (_, _) => 0,
        })
        .sum();
    println!("{ans}");
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
