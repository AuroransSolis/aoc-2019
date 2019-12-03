use std::io::{stdin, Read};
use std::collections::{HashSet, HashMap};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    part_1(&input);
    part_2(&input);
}

fn part_1(input: &str) {
    let lines = input.lines().collect::<Vec<_>>();
    let mut line_points = vec![HashSet::new(); lines.len()];
    for (i, line) in lines.into_iter().enumerate() {
        let moves = line.split(',').collect::<Vec<_>>();
        let mut x = 0;
        let mut y = 0;
        for m in moves {
            let (d, amt) = m.split_at(1);
            let amt = amt.parse::<i64>().unwrap();
            match d {
                "U" => {
                    for y in y + 1..=y + amt {
                        line_points[i].insert((x, y));
                    }
                    y += amt;
                },
                "R" => {
                    for x in x + 1..=x + amt {
                        line_points[i].insert((x, y));
                    }
                    x += amt;
                },
                "D" => {
                    for y in y - amt..y {
                        line_points[i].insert((x, y));
                    }
                    y -= amt;
                },
                "L" => {
                    for x in x - amt..x {
                        line_points[i].insert((x, y));
                    }
                    x -= amt;
                },
                _ => panic!("Invalid direction"),
            }
        }
    }
    let mut shortest = std::i64::MAX;
    let mut shortest_x = shortest;
    let mut shortest_y = shortest;
    for &(x, y) in line_points[1].iter() {
        if x == y && x == 0 {
            continue;
        }
        if line_points[0].contains(&(x, y)) {
            if x.abs() + y.abs() < shortest {
                shortest = x.abs() + y.abs();
                shortest_x = x;
                shortest_y = y;
            }
        }
    }
    println!("shortest: {} ({}, {})", shortest, shortest_x, shortest_y);
}

fn part_2(input: &str) {
    let lines = input.lines().collect::<Vec<_>>();
    let mut line_points = vec![HashMap::new(); lines.len()];
    for i in 0..lines.len() {
        line_points[i].insert((0, 0), 0);
    }
    for (i, line) in lines.into_iter().enumerate() {
        let moves = line.split(',').collect::<Vec<_>>();
        let mut x = 0;
        let mut y = 0;
        for m in moves {
            let (d, amt) = m.split_at(1);
            let amt = amt.parse::<i64>().unwrap();
            match d {
                "U" => {
                    for y in y + 1..=y + amt {
                        let steps_to = line_points[i].len();
                        line_points[i].insert((x, y), steps_to);
                    }
                    y += amt;
                },
                "R" => {
                    for x in x + 1..=x + amt {
                        let steps_to = line_points[i].len();
                        line_points[i].insert((x, y), steps_to);
                    }
                    x += amt;
                },
                "D" => {
                    for y in (y - amt..y).rev() {
                        let steps_to = line_points[i].len();
                        line_points[i].insert((x, y), steps_to);
                    }
                    y -= amt;
                },
                "L" => {
                    for x in (x - amt..x).rev() {
                        let steps_to = line_points[i].len();
                        line_points[i].insert((x, y), steps_to);
                    }
                    x -= amt;
                },
                _ => panic!("Invalid direction"),
            }
        }
    }
    let mut shortest = std::usize::MAX;
    for (point, &l2_steps) in line_points[1].iter() {
        if *point == (0, 0) {
            continue;
        }
        if let Some(&l1_steps) = line_points[0].get(point) {
            shortest = shortest.min(l1_steps + l2_steps);
        }
    }
    println!("shortest: {}", shortest);
}