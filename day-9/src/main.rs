use std::{collections::HashSet, fs, iter};

fn get_moves(input: Vec<String>) -> Vec<(char, i32)> {
    input
        .iter()
        .map(|line| line.split_at(line.find(" ").unwrap()))
        .map(|(a, b)| (a.parse().unwrap(), b[1..].parse().unwrap()))
        .collect()
}

fn dist(a: (i32, i32), b: (i32, i32)) -> i32 {
    (a.0 - b.0).pow(2) + (a.1 - b.1).pow(2)
}

fn compute_new_head_pos(dir: char, head: (i32, i32)) -> (i32, i32) {
    match dir {
        'U' => (head.0, head.1 + 1),
        'D' => (head.0, head.1 - 1),
        'L' => (head.0 - 1, head.1),
        'R' => (head.0 + 1, head.1),
        _ => (0, 0),
    }
}

fn compute_new_tail_pos(head: (i32, i32), tail: (i32, i32)) -> (i32, i32) {
    let mut new_tail_pos = tail;

    if dist(head, tail) > 2 {
        if head.0 < new_tail_pos.0 {
            new_tail_pos = (new_tail_pos.0 - 1, new_tail_pos.1);
        } else if head.0 > new_tail_pos.0 {
            new_tail_pos = (new_tail_pos.0 + 1, new_tail_pos.1);
        }

        if head.1 < new_tail_pos.1 {
            new_tail_pos = (new_tail_pos.0, new_tail_pos.1 - 1);
        } else if head.1 > new_tail_pos.1 {
            new_tail_pos = (new_tail_pos.0, new_tail_pos.1 + 1);
        }
    }

    new_tail_pos
}

fn solve_1(input: Vec<String>) {
    let moves = get_moves(input);

    let mut tail_visited: HashSet<(i32, i32)> = HashSet::new();

    let mut head = (0, 0);
    let mut tail = (0, 0);

    for (dir, num_steps) in moves {
        for _ in 0..num_steps {
            head = compute_new_head_pos(dir, head);
            tail = compute_new_tail_pos(head, tail);
            tail_visited.insert(tail);
        }
    }

    println!("{}", tail_visited.len())
}

fn solve_2(input: Vec<String>) {
    let moves = get_moves(input);

    let mut tail_visited: HashSet<(i32, i32)> = HashSet::new();

    let mut body: Vec<_> = iter::repeat((0, 0)).take(10).collect();

    for (dir, num_steps) in moves {
        for _ in 0..num_steps {
            body[0] = compute_new_head_pos(dir, body[0]);
            for i in 1..10 {
                body[i] = compute_new_tail_pos(body[i - 1], body[i]);
            }
            tail_visited.insert(*body.last().unwrap());
        }
    }

    println!("{}", tail_visited.len())
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
