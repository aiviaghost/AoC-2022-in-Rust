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

fn solve_1(input: Vec<String>) {
    let moves = get_moves(input);

    let mut tail_visited: HashSet<(i32, i32)> = HashSet::new();

    let mut head = (0, 0);
    let mut tail = (0, 0);

    for (dir, num_steps) in moves {
        match dir {
            'U' => {
                for _ in 0..num_steps {
                    head = (head.0, head.1 + 1);
                    if dist(head, tail) > 2 {
                        tail = (tail.0, tail.1 + 1);
                        if head.0 < tail.0 {
                            tail = (tail.0 - 1, tail.1);
                        } else if head.0 > tail.0 {
                            tail = (tail.0 + 1, tail.1);
                        }
                    }
                    tail_visited.insert(tail);
                }
            }
            'D' => {
                for _ in 0..num_steps {
                    head = (head.0, head.1 - 1);
                    if dist(head, tail) > 2 {
                        tail = (tail.0, tail.1 - 1);
                        if head.0 < tail.0 {
                            tail = (tail.0 - 1, tail.1);
                        } else if head.0 > tail.0 {
                            tail = (tail.0 + 1, tail.1);
                        }
                    }
                    tail_visited.insert(tail);
                }
            }
            'L' => {
                for _ in 0..num_steps {
                    head = (head.0 - 1, head.1);
                    if dist(head, tail) > 2 {
                        tail = (tail.0 - 1, tail.1);
                        if head.1 < tail.1 {
                            tail = (tail.0, tail.1 - 1);
                        } else if head.1 > tail.1 {
                            tail = (tail.0, tail.1 + 1);
                        }
                    }
                    tail_visited.insert(tail);
                }
            }
            'R' => {
                for _ in 0..num_steps {
                    head = (head.0 + 1, head.1);
                    if dist(head, tail) > 2 {
                        tail = (tail.0 + 1, tail.1);
                        if head.1 < tail.1 {
                            tail = (tail.0, tail.1 - 1);
                        } else if head.1 > tail.1 {
                            tail = (tail.0, tail.1 + 1);
                        }
                    }
                    tail_visited.insert(tail);
                }
            }
            _ => println!("hello"),
        }
    }

    println!("{}", tail_visited.len())
}

fn solve_2(input: Vec<String>) {
    let moves = get_moves(input);

    let mut tail_visited: HashSet<(i32, i32)> = HashSet::new();

    let mut body: Vec<_> = iter::repeat((0, 0)).take(10).collect();

    for (dir, num_steps) in moves {
        match dir {
            'U' => {
                for _ in 0..num_steps {
                    body[0] = (body[0].0, body[0].1 + 1);
                    for i in 1..10 {
                        if dist(body[i - 1], body[i]) > 2 {
                            if body[i - 1].0 < body[i].0 {
                                body[i] = (body[i].0 - 1, body[i].1);
                            } else if body[i - 1].0 > body[i].0 {
                                body[i] = (body[i].0 + 1, body[i].1);
                            }

                            if body[i - 1].1 < body[i].1 {
                                body[i] = (body[i].0, body[i].1 - 1);
                            } else if body[i - 1].1 > body[i].1 {
                                body[i] = (body[i].0, body[i].1 + 1);
                            }
                        }
                        tail_visited.insert(*body.last().unwrap());
                    }
                }
            }
            'D' => {
                for _ in 0..num_steps {
                    body[0] = (body[0].0, body[0].1 - 1);
                    for i in 1..10 {
                        if dist(body[i - 1], body[i]) > 2 {
                            if body[i - 1].0 < body[i].0 {
                                body[i] = (body[i].0 - 1, body[i].1);
                            } else if body[i - 1].0 > body[i].0 {
                                body[i] = (body[i].0 + 1, body[i].1);
                            }

                            if body[i - 1].1 < body[i].1 {
                                body[i] = (body[i].0, body[i].1 - 1);
                            } else if body[i - 1].1 > body[i].1 {
                                body[i] = (body[i].0, body[i].1 + 1);
                            }
                        }
                        tail_visited.insert(*body.last().unwrap());
                    }
                }
            }
            'L' => {
                for _ in 0..num_steps {
                    body[0] = (body[0].0 - 1, body[0].1);
                    for i in 1..10 {
                        if dist(body[i - 1], body[i]) > 2 {
                            if body[i - 1].0 < body[i].0 {
                                body[i] = (body[i].0 - 1, body[i].1);
                            } else if body[i - 1].0 > body[i].0 {
                                body[i] = (body[i].0 + 1, body[i].1);
                            }

                            if body[i - 1].1 < body[i].1 {
                                body[i] = (body[i].0, body[i].1 - 1);
                            } else if body[i - 1].1 > body[i].1 {
                                body[i] = (body[i].0, body[i].1 + 1);
                            }
                        }
                        tail_visited.insert(*body.last().unwrap());
                    }
                }
            }
            'R' => {
                for _ in 0..num_steps {
                    body[0] = (body[0].0 + 1, body[0].1);
                    for i in 1..10 {
                        if dist(body[i - 1], body[i]) > 2 {
                            if body[i - 1].0 < body[i].0 {
                                body[i] = (body[i].0 - 1, body[i].1);
                            } else if body[i - 1].0 > body[i].0 {
                                body[i] = (body[i].0 + 1, body[i].1);
                            }

                            if body[i - 1].1 < body[i].1 {
                                body[i] = (body[i].0, body[i].1 - 1);
                            } else if body[i - 1].1 > body[i].1 {
                                body[i] = (body[i].0, body[i].1 + 1);
                            }
                        }
                        tail_visited.insert(*body.last().unwrap());
                    }
                }
            }
            _ => println!("hello"),
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
