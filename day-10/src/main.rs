use std::{fs, iter};

fn make_instructions(input: Vec<String>) -> Vec<(String, i32)> {
    input
        .iter()
        .map(|line| line.split(" "))
        .map(|mut x| {
            let op = x.next().unwrap().to_string();
            if op == "noop" {
                (op, 0)
            } else {
                let v = x.next().unwrap().parse().unwrap();
                (op, v)
            }
        })
        .collect()
}

fn solve_1(input: Vec<String>) {
    let mut x = 1;
    let mut clock = 1;

    let clock_times: Vec<i32> = iter::repeat(40)
        .scan(20, |acc, step| {
            let res = *acc;
            *acc = *acc + step;
            Some(res)
        })
        .take_while(|x| *x <= 220)
        .collect();

    let mut ans = 0;

    let mut do_cycle = |x| {
        if clock_times.contains(&clock) {
            ans += clock * x;
        }
        clock += 1;
    };

    for (op, v) in make_instructions(input) {
        if op == "noop" {
            do_cycle(x);
        } else {
            do_cycle(x);
            do_cycle(x);
            x += v;
        }
    }
    println!("{ans}")
}

fn overlaps_sprite(screen_x: i32, sprite_pos: i32) -> bool {
    screen_x == (sprite_pos - 1) || screen_x == sprite_pos || screen_x == (sprite_pos + 1)
}

fn next_screen_pos(curr_pos: (usize, usize)) -> (usize, usize) {
    let mut x = curr_pos.0;
    let mut y = curr_pos.1;

    if x + 1 == 40 {
        x = 0;
        y += 1;
    } else {
        x += 1;
    }

    (x, y)
}

fn solve_2(input: Vec<String>) {
    let mut x = 1;
    let mut screen_pos: (usize, usize) = (0, 0);

    let mut screen: Vec<Vec<char>> = vec![vec![0 as char; 40]; 6];

    let mut do_cycle = |x| {
        screen[screen_pos.1][screen_pos.0] = if overlaps_sprite(screen_pos.0 as i32, x) {
            '#'
        } else {
            '.'
        };
        screen_pos = next_screen_pos(screen_pos);
    };

    for (op, v) in make_instructions(input) {
        if op == "noop" {
            do_cycle(x);
        } else {
            do_cycle(x);
            do_cycle(x);
            x += v;
        }
    }

    screen
        .iter()
        .for_each(|line| println!("{}", line.iter().collect::<String>()))
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
