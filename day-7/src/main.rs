use std::{fs, iter::Peekable};

fn traverse<'a, I>(rest: &mut Peekable<I>, min_required: i32) -> (i32, i32, i32)
where
    I: Iterator<Item = &'a String>,
{
    rest.next();

    let mut file_size_sum = 0;
    while let Some(line) = rest.peek() {
        if !line.starts_with("$") {
            if !line.starts_with("dir") {
                file_size_sum += rest
                    .next()
                    .unwrap()
                    .split(" ")
                    .next()
                    .unwrap()
                    .parse::<i32>()
                    .unwrap();
            } else {
                rest.next();
            }
        } else {
            break;
        }
    }

    let mut ans = 0;
    let mut candidate: Option<i32> = None;

    while let Some(line) = rest.next() {
        if line.starts_with("$ cd") && !line.ends_with("..") {
            let (nested_sum, nested_ans, new_candidate) = traverse(rest, min_required);
            file_size_sum += nested_sum;
            if new_candidate >= min_required {
                if let Some(curr) = candidate {
                    candidate = Some(curr.min(new_candidate));
                } else {
                    candidate = Some(new_candidate);
                }
            }
            ans += nested_ans;
        } else {
            break;
        }
    }

    if file_size_sum <= 100000 {
        ans += file_size_sum;
    }

    (file_size_sum, ans, *candidate.get_or_insert(file_size_sum))
}

fn solve_1(input: Vec<String>) {
    let mut data = input.iter().peekable();
    data.next();
    let (_, ans, _) = traverse(&mut data, 0);

    println!("{ans}")
}

fn solve_2(input: Vec<String>) {
    let mut data = input.iter().peekable();
    data.next();
    let (total_used, _, _) = traverse(&mut data, 0);

    let min_required = 30000000 - (70000000 - total_used);

    data = input.iter().peekable();
    data.next();
    let (_, _, ans) = traverse(&mut data, min_required);

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
