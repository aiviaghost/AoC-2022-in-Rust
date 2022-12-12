use std::{collections::VecDeque, fs};

fn make_int_grid(input: Vec<String>) -> Vec<Vec<i32>> {
    input
        .iter()
        .map(|line| {
            line.to_lowercase()
                .bytes()
                .map(|b| (b - ('a' as u8)) as i32)
                .collect()
        })
        .collect()
}

fn find_start_and_end(input: &Vec<String>) -> ((usize, usize), (usize, usize)) {
    let mut start: (usize, usize) = (0, 0);
    let mut end: (usize, usize) = (0, 0);

    for i in 0..input.len() {
        for j in 0..input[i].len() {
            if input[i].as_bytes()[j] as char == 'S' {
                start = (j, i);
            } else if input[i].as_bytes()[j] as char == 'E' {
                end = (j, i);
            }
        }
    }

    (start, end)
}

fn bfs(grid: Vec<Vec<i32>>, starts: Vec<(usize, usize)>, end: (usize, usize)) -> i32 {
    let (n, m) = (grid.len(), grid[0].len());
    let mut dist = vec![vec![-1; m]; n];
    let mut q: VecDeque<(usize, usize)> = VecDeque::new();
    for start in &starts {
        dist[start.1][start.0] = 0;
        q.push_back(*start);
    }
    let nexts: Vec<(i32, i32)> = vec![(0, -1), (0, 1), (1, 0), (-1, 0)];
    while let Some((cx, cy)) = q.pop_front() {
        for (dx, dy) in &nexts {
            let nx = cx as i32 + *dx;
            let ny = cy as i32 + *dy;
            if 0 <= nx && nx < m as i32 && 0 <= ny && ny < n as i32 {
                let nx = nx as usize;
                let ny = ny as usize;
                if dist[ny][nx] == -1 && grid[ny][nx] <= grid[cy][cx] + 1 {
                    dist[ny][nx] = dist[cy][cx] + 1;
                    q.push_back((nx, ny));
                }
            }
        }
    }
    dist[end.1][end.0]
}

fn solve_1(input: Vec<String>) {
    let (start, end) = find_start_and_end(&input);

    let mut grid = make_int_grid(input);
    grid[end.1][end.0] = 25;

    let starts = vec![start];

    let ans = bfs(grid, starts, end);

    println!("{}", ans)
}

fn solve_2(input: Vec<String>) {
    let (_, end) = find_start_and_end(&input);

    let mut grid = make_int_grid(input);
    grid[end.1][end.0] = 25;

    let mut starts: Vec<(usize, usize)> = Vec::new();
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] == 0 {
                starts.push((j, i));
            }
        }
    }

    let ans = bfs(grid, starts, end);

    println!("{}", ans)
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
