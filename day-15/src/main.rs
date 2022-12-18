use std::{collections::HashSet, fs};

struct Sensor {
    pos: (i32, i32),
    closest_beacon: (i32, i32),
}

fn parse_sensors(input: Vec<String>) -> Vec<Sensor> {
    input
        .iter()
        .map(|line| {
            let mut parts = line.split(" ");

            parts.next();
            parts.next();

            let sx = parts.next().unwrap()[2..].to_string();
            let sx: i32 = sx[..sx.len() - 1].parse().unwrap();

            let sy = parts.next().unwrap()[2..].to_string();
            let sy: i32 = sy[..sy.len() - 1].parse().unwrap();

            parts.next();
            parts.next();
            parts.next();
            parts.next();

            let bx = parts.next().unwrap()[2..].to_string();
            let bx: i32 = bx[..bx.len() - 1].parse().unwrap();

            let by = parts.next().unwrap()[2..].to_string();
            let by: i32 = by.parse().unwrap();

            Sensor {
                pos: (sx, sy),
                closest_beacon: (bx, by),
            }
        })
        .collect()
}

fn manhattan_dist(p1: (i32, i32), p2: (i32, i32)) -> i32 {
    (p1.0 - p2.0).abs() + (p1.1 - p2.1).abs()
}

fn query_pos(sensors: &Vec<Sensor>, pos: (i32, i32)) -> bool {
    sensors
        .iter()
        .map(|sensor| {
            if sensor.closest_beacon == pos {
                return false;
            }
            manhattan_dist(pos, sensor.pos) <= manhattan_dist(sensor.pos, sensor.closest_beacon)
        })
        .fold(false, |curr_state, next| curr_state || next)
}

fn solve_1(input: Vec<String>) {
    let sensors = parse_sensors(input);

    let ans: i32 = (-1_000_000..5_000_000)
        .map(|x| query_pos(&sensors, (x, 2_000_000)) as i32)
        .sum();

    println!("{ans}")
}

fn trace_sides(sensor: &Sensor) -> Vec<(i32, i32)> {
    let beacon_dist = manhattan_dist(sensor.pos, sensor.closest_beacon);
    let top = (sensor.pos.0, sensor.pos.1 + beacon_dist + 1);
    let mut points = vec![top];
    let mut dir = (-1, -1);
    let mut p = (top.0 + dir.0, top.1 + dir.1);
    while p != top {
        points.push(p);
        let next = (p.0 + dir.0, p.1 + dir.1);
        if manhattan_dist(sensor.pos, next) > beacon_dist + 1 {
            dir = match dir {
                (-1, -1) => (1, -1),
                (1, -1) => (1, 1),
                (1, 1) => (-1, 1),
                (-1, 1) => (-1, -1),
                (_, _) => unreachable!(),
            }
        }
        p = (p.0 + dir.0, p.1 + dir.1);
    }
    points
}

fn solve_2(input: Vec<String>) {
    let sensors = parse_sensors(input);
    let points: HashSet<_> = sensors
        .iter()
        .map(|sensor| trace_sides(sensor))
        .flatten()
        .collect();

    let valid = points
        .iter()
        .filter(|p| 0 <= p.0 && p.0 <= 4000000 && 0 <= p.1 && p.1 <= 4000000)
        .map(|p| (p, query_pos(&sensors, *p)))
        .find(|(_p, valid)| !*valid)
        .unwrap()
        .0;

    let ans: i64 = valid.0 as i64 * 4000000 + valid.1 as i64;

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
