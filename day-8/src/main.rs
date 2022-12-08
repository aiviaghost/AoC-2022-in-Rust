use std::fs;

fn make_grid(input: &Vec<String>) -> Vec<Vec<i32>> {
    input
        .iter()
        .map(|line| {
            line.bytes()
                .map(|x| x.to_string().parse().unwrap())
                .collect()
        })
        .collect()
}

fn solve_1(input: Vec<String>) {
    let grid = make_grid(&input);

    let mut ans = 0;

    let n = grid.len();
    for i in 1..n - 1 {
        for j in 1..n - 1 {
            let curr = grid[i][j];
            let mut visible = false;

            if (0..i).map(|k| grid[k][j]).max().unwrap() < curr {
                visible = true;
            }

            if (i + 1..n).map(|k| grid[k][j]).max().unwrap() < curr {
                visible = true;
            }

            if (0..j).map(|k| grid[i][k]).max().unwrap() < curr {
                visible = true;
            }

            if (j + 1..n).map(|k| grid[i][k]).max().unwrap() < curr {
                visible = true;
            }

            if visible {
                ans += 1;
            }
        }
    }

    println!("{}", 4 * (n - 1) + ans)
}

fn solve_2(input: Vec<String>) {
    let grid = make_grid(&input);

    let mut ans = 0;

    let n = grid.len();
    for i in 0..n {
        for j in 0..n {
            let curr_h = grid[i][j];

            let top = {
                let mut k = 1;
                loop {
                    if i < k {
                        k -= 1;
                        break;
                    } else if grid[i - k][j] >= curr_h {
                        break;
                    }
                    k += 1;
                }
                k
            };

            let bottom = {
                let mut k = 1;
                loop {
                    if i + k >= n {
                        k -= 1;
                        break;
                    } else if grid[i + k][j] >= curr_h {
                        break;
                    }
                    k += 1;
                }
                k
            };

            let left = {
                let mut k = 1;
                loop {
                    if j < k {
                        k -= 1;
                        break;
                    } else if grid[i][j - k] >= curr_h {
                        break;
                    }
                    k += 1;
                }
                k
            };

            let right = {
                let mut k = 1;
                loop {
                    if j + k >= n {
                        k -= 1;
                        break;
                    } else if grid[i][j + k] >= curr_h {
                        break;
                    }
                    k += 1;
                }
                k
            };

            ans = ans.max(top * bottom * left * right);
        }
    }

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
