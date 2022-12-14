use std::{cmp::Ordering, fs, iter::Peekable};

#[derive(Debug, Clone, PartialEq, Eq)]
struct Node {
    items: Vec<Node>,
    item: Option<i32>,
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (
                Node {
                    items: _,
                    item: Some(a),
                },
                Node {
                    items: _,
                    item: Some(b),
                },
            ) => a.cmp(b),
            (
                Node {
                    items: a,
                    item: None,
                },
                Node {
                    items: b,
                    item: None,
                },
            ) => a.cmp(b),
            (
                Node {
                    items: _,
                    item: Some(a),
                },
                Node {
                    items: b,
                    item: None,
                },
            ) => item_to_vec(*a).cmp(b),
            (
                Node {
                    items: a,
                    item: None,
                },
                Node {
                    items: _,
                    item: Some(b),
                },
            ) => a.cmp(&item_to_vec(*b)),
        }
    }
}

fn item_to_vec(x: i32) -> Vec<Node> {
    vec![Node {
        items: Vec::new(),
        item: Some(x),
    }]
}

fn build_tree<I>(rest: &mut Peekable<I>) -> Node
where
    I: Iterator<Item = char>,
{
    let mut node = Node {
        items: Vec::new(),
        item: None,
    };

    while let Some(&c) = rest.peek() {
        if c == ']' {
            rest.next();
            break;
        }
        if c == '[' {
            rest.next();
            node.items.push(build_tree(rest));
        } else if c == ',' {
            rest.next();
        } else {
            let mut num: Vec<char> = Vec::new();
            while rest.peek().unwrap().to_string().parse::<i32>().is_ok() {
                num.push(rest.next().unwrap());
            }
            let num = num.iter().collect::<String>().parse::<i32>();
            node.items.push(Node {
                items: Vec::new(),
                item: if num.is_ok() {
                    Some(num.unwrap())
                } else {
                    None
                },
            });
        }
    }

    node
}

fn solve_1(input: Vec<String>) {
    let ans: usize = input
        .iter()
        .map(|lines| lines.split("\n"))
        .enumerate()
        .map(|(i, mut lines)| {
            let mut line_1 = lines.next().unwrap().chars().peekable();
            let mut line_2 = lines.next().unwrap().chars().peekable();

            let tree_1 = build_tree(&mut line_1).items[0].clone();
            let tree_2 = build_tree(&mut line_2).items[0].clone();

            if tree_1 < tree_2 {
                i + 1
            } else {
                0
            }
        })
        .sum();

    println!("{ans}")
}

fn solve_2(mut input: Vec<String>) {
    input.push(String::from("[[2]]\n[[6]]"));

    let mut data: Vec<_> = input
        .iter()
        .map(|lines| lines.split("\n"))
        .map(|mut lines| {
            let mut line_1 = lines.next().unwrap().chars().peekable();
            let mut line_2 = lines.next().unwrap().chars().peekable();

            let tree_1 = build_tree(&mut line_1).items[0].clone();
            let tree_2 = build_tree(&mut line_2).items[0].clone();

            vec![tree_1, tree_2]
        })
        .flatten()
        .enumerate()
        .map(|(i, node)| (node, i))
        .collect();

    let n = data.len();
    data.sort_unstable();

    let mut ans: Vec<usize> = Vec::new();

    data.iter().enumerate().for_each(|(idx, (_, i))| {
        if *i == n - 1 || *i == n - 2 {
            ans.push(idx + 1);
        }
    });

    println!("{}", ans.iter().fold(1, |acc, next| acc * next))
}

fn main() {
    let input: Vec<String> = fs::read_to_string("input.txt")
        .unwrap()
        .split_terminator("\n\n")
        .map(|x| x.to_string())
        .collect();

    solve_1(input.clone());
    solve_2(input.clone());
}
