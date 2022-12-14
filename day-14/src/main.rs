use std::fs;

fn make_lines(input: Vec<String>) -> Vec<Vec<(usize, usize)>> {
    input
        .iter()
        .map(|line| {
            line.split(" -> ")
                .map(|coord| {
                    let mut s = coord.split(",");
                    let x = s.next().unwrap().parse::<usize>().unwrap();
                    let y = s.next().unwrap().parse::<usize>().unwrap();
                    (x, y)
                })
                .collect()
        })
        .collect()
}

fn find_min_xy(lines: &Vec<Vec<(usize, usize)>>) -> (usize, usize) {
    lines
        .iter()
        .flatten()
        .fold((usize::MAX, usize::MAX), |acc, next| {
            (acc.0.min(next.0), acc.1.min(next.1))
        })
}

fn rescale(
    lines: Vec<Vec<(usize, usize)>>,
    min_x: usize,
    min_y: usize,
) -> Vec<Vec<(usize, usize)>> {
    lines
        .iter()
        .map(|line| line.iter().map(|p| (p.0 - min_x, p.1 - min_y)).collect())
        .collect()
}

fn make_grid(
    n: usize,
    m: usize,
    y_offset: usize,
    lines: Vec<Vec<(usize, usize)>>,
) -> Vec<Vec<char>> {
    let mut grid = vec![vec!['.'; m]; y_offset + n];
    lines.iter().for_each(|line| {
        line.windows(2)
            .map(|x| (x[0], x[1]))
            .for_each(|((x1, y1), (x2, y2))| {
                if x1 == x2 {
                    for y in y1.min(y2)..=y1.max(y2) {
                        grid[y_offset + y][x1] = '#';
                    }
                } else {
                    for x in x1.min(x2)..=x1.max(x2) {
                        grid[y_offset + y1][x] = '#';
                    }
                }
            })
    });
    grid
}

fn solve_1(input: Vec<String>) {
    let mut lines = make_lines(input);

    let (min_x, min_y) = find_min_xy(&lines);

    let start_x = 500 - min_x;
    lines = rescale(lines, min_x, min_y);

    let (m, n) = lines.iter().flatten().fold((0, 0), |acc, next| {
        (acc.0.max(next.0 + 1), acc.1.max(next.1 + 1))
    });

    let y_offset = m;

    let mut grid = make_grid(n, m, y_offset, lines);

    let mut ans = 0;
    'outer: loop {
        let (mut cx, mut cy) = (start_x, 0 as usize);
        loop {
            if cy + 1 == y_offset + n {
                break 'outer;
            } else if grid[cy + 1][cx] == '.' {
                cy += 1;
                continue;
            }

            if cx == 0 {
                break 'outer;
            } else if grid[cy + 1][cx - 1] == '.' {
                cx -= 1;
                cy += 1;
                continue;
            }

            if cx + 1 == m {
                break 'outer;
            } else if grid[cy + 1][cx + 1] == '.' {
                cx += 1;
                cy += 1;
                continue;
            }

            grid[cy][cx] = 'O';
            ans += 1;
            break;
        }
    }

    println!("{ans}")
}

fn solve_2(input: Vec<String>) {
    let mut lines = make_lines(input);

    let (min_x, min_y) = find_min_xy(&lines);

    lines = rescale(lines, min_x, min_y);

    let (mut m, mut n) = lines.iter().flatten().fold((0, 0), |acc, next| {
        (acc.0.max(next.0 + 1), acc.1.max(next.1 + 1))
    });

    let start_x = 500 - min_x + 3 * m;

    lines = lines
        .iter()
        .map(|line| line.iter().map(|p| (p.0 + 3 * m, p.1)).collect())
        .collect();

    lines.push(vec![(0, n + 1), (7 * m, n + 1)]);
    n += 2;
    m = 7 * m + 1;

    let y_offset = n;

    let mut grid = make_grid(n, m, y_offset, lines);

    let mut ans = 0;
    loop {
        let (mut cx, mut cy) = (start_x, y_offset - min_y);
        if grid[cy][cx] == 'O' {
            break;
        }
        loop {
            if grid[cy + 1][cx] == '.' {
                cy += 1;
                continue;
            }

            if grid[cy + 1][cx - 1] == '.' {
                cx -= 1;
                cy += 1;
                continue;
            }

            if grid[cy + 1][cx + 1] == '.' {
                cx += 1;
                cy += 1;
                continue;
            }

            grid[cy][cx] = 'O';
            ans += 1;
            break;
        }
    }

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
