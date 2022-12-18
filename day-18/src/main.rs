use std::{collections::VecDeque, fs};

fn solve_1(input: Vec<String>) {
    let mut cubes: Vec<(usize, usize, usize)> = input
        .iter()
        .map(|line| {
            line.split(",")
                .map(|x| x.parse().unwrap())
                .collect::<Vec<_>>()
        })
        .map(|item| (item[0], item[1], item[2]))
        .collect();

    let (min_x, min_y, min_z) = cubes.iter().fold(
        (usize::MAX, usize::MAX, usize::MAX),
        |(cx, cy, cz), (nx, ny, nz)| (cx.min(*nx), cy.min(*ny), cz.min(*nz)),
    );
    cubes = cubes
        .iter()
        .map(|(x, y, z)| (x - min_x, y - min_y, z - min_z))
        .collect();
    let (max_x, max_y, max_z) = cubes.iter().fold((0, 0, 0), |(cx, cy, cz), (nx, ny, nz)| {
        (cx.max(nx + 1), cy.max(ny + 1), cz.max(nz + 1))
    });

    let mut grid = vec![vec![vec![false; max_x]; max_y]; max_z];
    for (x, y, z) in cubes {
        grid[z][y][x] = true;
    }

    let neighbours = vec![
        (0, 0, 1),
        (0, 0, -1),
        (0, 1, 0),
        (0, -1, 0),
        (1, 0, 0),
        (-1, 0, 0),
    ];

    let mut ans = 0;
    let mut vis = vec![vec![vec![false; max_x]; max_y]; max_z];
    vis[0][0][0] = true;
    let mut q: VecDeque<(usize, usize, usize)> = VecDeque::new();
    q.push_back((0, 0, 0));
    while let Some((cx, cy, cz)) = q.pop_front() {
        if grid[cz][cy][cx] {
            ans += 6;
        }
        for (dx, dy, dz) in &neighbours {
            let (nx, ny, nz) = (cx as i32 + dx, cy as i32 + dy, cz as i32 + dz);
            if 0 <= nx
                && nx < max_x as i32
                && 0 <= ny
                && ny < max_y as i32
                && 0 <= nz
                && nz < max_z as i32
            {
                let (nx, ny, nz) = (nx as usize, ny as usize, nz as usize);
                if grid[cz][cy][cx] && grid[nz][ny][nx] {
                    ans -= 1;
                }
                if !vis[nz][ny][nx] {
                    vis[nz][ny][nx] = true;
                    q.push_back((nx, ny, nz));
                }
            }
        }
    }

    println!("{ans}")
}

fn solve_2(input: Vec<String>) {
    let mut cubes: Vec<(usize, usize, usize)> = input
        .iter()
        .map(|line| {
            line.split(",")
                .map(|x| x.parse().unwrap())
                .collect::<Vec<_>>()
        })
        .map(|item| (item[0], item[1], item[2]))
        .collect();

    let (min_x, min_y, min_z) = cubes.iter().fold(
        (usize::MAX, usize::MAX, usize::MAX),
        |(cx, cy, cz), (nx, ny, nz)| (cx.min(*nx), cy.min(*ny), cz.min(*nz)),
    );
    cubes = cubes
        .iter()
        .map(|(x, y, z)| (x - min_x + 1, y - min_y + 1, z - min_z + 1))
        .collect();
    let (max_x, max_y, max_z) = cubes.iter().fold((0, 0, 0), |(cx, cy, cz), (nx, ny, nz)| {
        (cx.max(nx + 3), cy.max(ny + 3), cz.max(nz + 3))
    });

    let mut grid = vec![vec![vec![false; max_x]; max_y]; max_z];
    for (x, y, z) in cubes {
        grid[z][y][x] = true;
    }

    let neighbours = vec![
        (0, 0, 1),
        (0, 0, -1),
        (0, 1, 0),
        (0, -1, 0),
        (1, 0, 0),
        (-1, 0, 0),
    ];

    let mut is_valid = vec![vec![vec![false; max_x]; max_y]; max_z];
    let mut vis = vec![vec![vec![false; max_x]; max_y]; max_z];
    vis[0][0][0] = true;
    let mut q: VecDeque<(usize, usize, usize)> = VecDeque::new();
    q.push_back((0, 0, 0));
    while let Some((cx, cy, cz)) = q.pop_front() {
        is_valid[cz][cy][cx] = true;
        for (dx, dy, dz) in &neighbours {
            let (nx, ny, nz) = (cx as i32 + dx, cy as i32 + dy, cz as i32 + dz);
            if 0 <= nx
                && nx < max_x as i32
                && 0 <= ny
                && ny < max_y as i32
                && 0 <= nz
                && nz < max_z as i32
            {
                let (nx, ny, nz) = (nx as usize, ny as usize, nz as usize);
                if !grid[nz][ny][nx] && !vis[nz][ny][nx] {
                    vis[nz][ny][nx] = true;
                    q.push_back((nx, ny, nz));
                }
            }
        }
    }

    let mut ans = 0;
    vis = vec![vec![vec![false; max_x]; max_y]; max_z];
    vis[0][0][0] = true;
    q.push_back((0, 0, 0));
    while let Some((cx, cy, cz)) = q.pop_front() {
        for (dx, dy, dz) in &neighbours {
            let (nx, ny, nz) = (cx as i32 + dx, cy as i32 + dy, cz as i32 + dz);
            if 0 <= nx
                && nx < max_x as i32
                && 0 <= ny
                && ny < max_y as i32
                && 0 <= nz
                && nz < max_z as i32
            {
                let (nx, ny, nz) = (nx as usize, ny as usize, nz as usize);
                if grid[cz][cy][cx] && is_valid[nz][ny][nx] {
                    ans += 1;
                }
                if !vis[nz][ny][nx] {
                    vis[nz][ny][nx] = true;
                    q.push_back((nx, ny, nz));
                }
            }
        }
    }

    print!("{ans}")
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
