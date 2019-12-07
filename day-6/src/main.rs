use std::io::{stdin, Read};
use std::collections::{HashMap, HashSet};

fn main() {
    let mut input = String::new();
    stdin().lock().read_to_string(&mut input).unwrap();
    let mut map = HashMap::new();
    let mut parents = HashSet::new();
    let mut children = HashSet::new();
    let mut depths = HashMap::new();
    for line in input.lines() {
        let mut split = line.split(')');
        let orbited = split.next().unwrap();
        let orbiter = split.next().unwrap();
        map.insert(orbiter, orbited);
        parents.insert(orbited);
        children.insert(orbiter);
        depths.entry(orbited).or_insert(0usize);
        let parent_depth = *depths.get(orbited).unwrap();
        depths.insert(orbiter, parent_depth + 1);
        println!("parent: {} ({}) | child: {} ({})", orbited, parent_depth, orbiter, parent_depth + 1);
    }
    let mut seen = HashSet::new();
    let mut total = map.len() - 1;
    println!("Depths:");
    for mass in depths.iter() {
        println!("{:?}", mass);
    }
    /*for child in map.values() {
        if seen.insert(child) {
            let mut at = child;
            let mut path_length = 0;
            while let Some(new) = map.get(at) {
                total += 1;
                at = new;
            }
        }
    }*/
    for base in children.difference(&parents) {
        let mut at = base;
        while let Some(new) = map.get(at) {
            at = new;
            if !seen.insert(new) {
                break;
            }
        }
        let parent_cost = *depths.get(map.get(base).unwrap()).unwrap();
        let mut base_cost = *depths.get(base).unwrap();
        if parent_cost > base_cost {
            base_cost = parent_cost + 1;
        }
        println!("adding depths for {} to {}", base, at);
        let start_depth = match base_cost {
            0 => 0,
            1 => 1,
            depth => depth * (depth - 1)
        };
        println!("indirects for {}: {}", base, start_depth / 2);
        let end_depth = match *depths.get(at).unwrap() {
            0 => 0,
            1 => 1,
            depth => depth * (depth - 1)
        };
        println!("indirects for {}: {}", at, end_depth / 2);
        println!("total new indirects: {}", start_depth.saturating_sub(end_depth) / 2);
        total += (start_depth.saturating_sub(end_depth)) / 2;
    }
    println!("p1: {}", total);
    let mut num_transfers = 0;
    let mut you_path = HashMap::new();
    let mut at = map.get("YOU").unwrap();
    you_path.insert(at, num_transfers);
    while let Some(parent) = map.get(at) {
        num_transfers += 1;
        you_path.insert(parent, num_transfers);
        at = parent;
    }
    at = map.get("SAN").unwrap();
    let mut san_transfers = 0;
    while let Some(parent) = map.get(at) {
        if let Some(&transfers) = you_path.get(at) {
            san_transfers += transfers;
            break;
        } else {
            at = parent;
            san_transfers += 1;
        }
    }
    println!("p2: {}", san_transfers);
}
