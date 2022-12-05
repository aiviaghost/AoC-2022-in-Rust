use std::{collections::VecDeque, fs};

fn setup(input: &Vec<String>) -> Vec<VecDeque<char>> {
    let mut stacks: Vec<VecDeque<char>> = std::iter::repeat(VecDeque::new()).take(9).collect();

    input
        .iter()
        .take_while(|line| !line.starts_with(" "))
        .for_each(|line| {
            line.bytes()
                .enumerate()
                .filter(|(i, _c)| i % 4 == 1)
                .map(|(_i, c)| c as char)
                .enumerate()
                .for_each(|(i, c)| {
                    if c != ' ' {
                        stacks[i].push_front(c)
                    }
                })
        });

    stacks
}

fn get_instructions(input: &Vec<String>) -> Vec<(usize, usize, usize)> {
    input
        .iter()
        .skip_while(|line| !line.starts_with("move"))
        .map(|line| {
            line.split(" ")
                .map(|word| word.parse::<usize>())
                .flatten()
                .collect::<Vec<_>>()
                .chunks(3)
                .map(|x| (x[0], x[1] - 1, x[2] - 1))
                .collect::<Vec<_>>()
        })
        .flatten()
        .collect()
}

fn solve_1(input: Vec<String>) {
    let mut stacks = setup(&input);

    get_instructions(&input).iter().for_each(|(num, from, to)| {
        for _i in 0..*num {
            let x = stacks[*from].pop_back().unwrap();
            stacks[*to].push_back(x);
        }
    });

    let ans: String = stacks.iter().map(|stack| stack.back().unwrap()).collect();

    println!("{ans}")
}

fn solve_2(input: Vec<String>) {
    let mut stacks = setup(&input);

    get_instructions(&input).iter().for_each(|(num, from, to)| {
        let mut temp: Vec<char> = Vec::new();
        for _i in 0..*num {
            let x = stacks[*from].pop_back().unwrap();
            temp.push(x);
        }
        temp.iter().rev().for_each(|x| stacks[*to].push_back(*x));
    });

    let ans: String = stacks.iter().map(|stack| stack.back().unwrap()).collect();

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
