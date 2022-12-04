use {std::fs, std::ops::RangeInclusive};

fn is_fully_overlapping(a: &RangeInclusive<i32>, b: &RangeInclusive<i32>) -> bool {
    a.contains(&b.start()) && a.contains(&b.end()) || b.contains(&a.start()) && b.contains(&a.end())
}

fn is_partially_overlapping(a: &RangeInclusive<i32>, b: &RangeInclusive<i32>) -> bool {
    a.contains(&b.start()) || a.contains(&b.end()) || b.contains(&a.start()) || b.contains(&a.end())
}

fn make_range(s: &str) -> RangeInclusive<i32> {
    let x: Vec<i32> = s.split("-").map(|x| x.parse().unwrap()).collect();
    x[0]..=x[1]
}

fn make_ranges(input: Vec<String>) -> Vec<(RangeInclusive<i32>, RangeInclusive<i32>)> {
    input
        .iter()
        .map(|line| {
            let (a, b) = line.split_at(line.find(",").unwrap());
            let a = make_range(a);
            let b = make_range(&b[1..]);
            (a, b)
        })
        .collect()
}

fn solve_1(input: Vec<String>) {
    let ans: i32 = make_ranges(input)
        .iter()
        .map(|(a, b)| is_fully_overlapping(a, b) as i32)
        .sum();
    println!("{ans}")
}

fn solve_2(input: Vec<String>) {
    let ans: i32 = make_ranges(input)
        .iter()
        .map(|(a, b)| is_partially_overlapping(a, b) as i32)
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
