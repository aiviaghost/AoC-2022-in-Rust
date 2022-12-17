use std::{collections::VecDeque, fs, ops};

#[derive(Copy, Clone)]
struct Dir {
    x: i32,
    y: i32,
}

#[derive(Copy, Clone)]
struct Pos {
    x: i32,
    y: i32,
}

impl ops::Add<Dir> for Pos {
    type Output = Pos;

    fn add(self, dir: Dir) -> Self::Output {
        Pos {
            x: self.x + dir.x,
            y: self.y + dir.y,
        }
    }
}

fn solve_1(input: Vec<String>) {
    let shapes = vec![
        (
            1,
            vec![
                Pos { x: 2, y: 0 },
                Pos { x: 3, y: 0 },
                Pos { x: 4, y: 0 },
                Pos { x: 5, y: 0 },
            ],
        ),
        (
            3,
            vec![
                Pos { x: 3, y: 0 },
                Pos { x: 2, y: 1 },
                Pos { x: 3, y: 1 },
                Pos { x: 4, y: 1 },
                Pos { x: 3, y: 2 },
            ],
        ),
        (
            3,
            vec![
                Pos { x: 4, y: 0 },
                Pos { x: 4, y: 1 },
                Pos { x: 4, y: 2 },
                Pos { x: 3, y: 2 },
                Pos { x: 2, y: 2 },
            ],
        ),
        (
            4,
            vec![
                Pos { x: 2, y: 0 },
                Pos { x: 2, y: 1 },
                Pos { x: 2, y: 2 },
                Pos { x: 2, y: 3 },
            ],
        ),
        (
            2,
            vec![
                Pos { x: 2, y: 0 },
                Pos { x: 3, y: 0 },
                Pos { x: 2, y: 1 },
                Pos { x: 3, y: 1 },
            ],
        ),
    ];
    let mut shape_index = 0;

    let flows: Vec<char> = input[0].chars().collect();
    let mut flow_index = 0;

    let mut grid: VecDeque<Vec<char>> = VecDeque::new();
    for _ in 0..2022 {
        let (shape_height, mut shape) = shapes[shape_index % shapes.len()].clone();
        shape_index += 1;

        for _ in 0..3 + shape_height {
            grid.push_front(vec!['.'; 7]);
        }

        loop {
            let dir = if flows[flow_index % flows.len()] == '<' {
                Dir { x: -1, y: 0 }
            } else {
                Dir { x: 1, y: 0 }
            };
            flow_index += 1;

            let mut next: Vec<_> = shape.iter().map(|pos| *pos + dir).collect();
            if !(next
                .iter()
                .map(|pos| 0 > pos.x || pos.x == 7 || grid[pos.y as usize][pos.x as usize] == '#')
                .fold(false, |state, next| state || next))
            {
                shape = next;
            }

            next = shape.iter().map(|pos| *pos + Dir { x: 0, y: 1 }).collect();
            if next
                .iter()
                .map(|pos| {
                    (pos.y as usize) == grid.len() || grid[pos.y as usize][pos.x as usize] == '#'
                })
                .fold(false, |state, next| state || next)
            {
                shape.iter().for_each(|Pos { x, y }| {
                    grid[*y as usize][*x as usize] = '#';
                });
                break;
            }
            shape = next;
        }

        while !grid[0].contains(&'#') {
            grid.pop_front();
        }
    }

    println!("{}", grid.len())
}

fn solve_2(input: Vec<String>) {}

fn main() {
    let input: Vec<String> = fs::read_to_string("input.txt")
        .unwrap()
        .split_terminator('\n')
        .map(|x| x.to_string())
        .collect();

    solve_1(input.clone());
    solve_2(input.clone());
}
