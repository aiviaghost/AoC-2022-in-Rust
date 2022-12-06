use {std::collections::HashSet, std::fs};

fn solve(input: &Vec<String>, seq_len: usize) -> i32 {
    (input[0]
        .bytes()
        .collect::<Vec<_>>()
        .windows(seq_len)
        .map(|w| w.iter().collect::<HashSet<_>>())
        .enumerate()
        .skip_while(|(_i, hs)| hs.len() < seq_len)
        .next()
        .unwrap()
        .0
        + seq_len) as i32
}

fn solve_1(input: Vec<String>) {
    let ans = solve(&input, 4);

    println!("{ans}")
}

fn solve_2(input: Vec<String>) {
    let ans = solve(&input, 14);

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
