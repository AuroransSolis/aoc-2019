use std::io::{stdin, BufRead};
use std::collections::HashMap;

fn main() {
    let mut line_points = Vec::new();
    for line in stdin().lock().lines() {
        let line = line.unwrap();
        let mut points = HashMap::new();
        let moves = line.split(',').collect::<Vec<_>>();
        let mut x = 0;
        let mut y = 0;
        let mut steps = 1;
        for m in moves {
            let (d, amt) = m.split_at(1);
            let amt = amt.parse::<i32>().unwrap();
            let instr = match d {
                "U" => yp1,
                "R" => xp1,
                "D" => ym1,
                "L" => xm1,
                _ => panic!("Invalid direction! Not [URDL]."),
            };
            for _ in 0..amt {
                instr(&mut x, &mut y);
                points.insert((x, y), steps);
                steps += 1;
            }
        }
        line_points.push(points);
    }
    let mut abs_closest = std::i32::MAX;
    let mut signal_closest = std::i32::MAX;
    for i in 0..line_points.len() - 1 {
        for j in i + 1..line_points.len() {
            for (&(x, y), &l1_cost) in line_points[i].iter() {
                if let Some(&l2_cost) = line_points[j].get(&(x, y)) {
                    abs_closest = abs_closest.min(x.abs() + y.abs());
                    signal_closest = signal_closest.min(l1_cost + l2_cost);
                }
            }
        }
    }
    println!("p1: {}", abs_closest);
    println!("p2: {}", signal_closest);
}

fn yp1(_x: &mut i32, y: &mut i32) {
    *y += 1;
}

fn xp1(x: &mut i32, _y: &mut i32) {
    *x += 1;
}

fn ym1(_x: &mut i32, y: &mut i32) {
    *y -= 1;
}

fn xm1(x: &mut i32, _y: &mut i32) {
    *x -= 1;
}