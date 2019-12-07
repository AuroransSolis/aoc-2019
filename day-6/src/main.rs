use std::io::{stdin, Read};
use std::collections::{HashMap, HashSet};

fn main() {
    let mut input = String::new();
    stdin().lock().read_to_string(&mut input).unwrap();
    // The key is the child and the value is the parent.
    let mut map = HashMap::new();
    // Keeps track of which masses have children.
    let mut parents = HashSet::new();
    // Keeps track of which masses have children.
    let mut children = HashSet::new();
    // The children.difference(&parents) gives the masses that have no children.
    // Keeps track of the reverse of map - the key is the parent and the value is all of that
    // parent's children.
    let mut reverse_map = HashMap::new();
    for line in input.lines() {
        let mut split = line.split(')');
        let orbited = split.next().unwrap();
        let orbiter = split.next().unwrap();
        map.insert(orbiter, orbited);
        parents.insert(orbited);
        children.insert(orbiter);
        reverse_map.entry(orbited).or_insert(Vec::new()).push(orbiter);
        reverse_map.entry(orbiter).or_insert(Vec::new());
    }
    // Depths keeps track of the number of nodes between COM and the other nodes.
    let mut depths = HashMap::new();
    find_depths(&reverse_map, &mut depths, "COM", 0);
    // Used to ensure that the indirects for each node are only counted once.
    let mut seen = HashSet::new();
    let mut total = map.len() - 1;
    for base in children.difference(&parents) {
        let mut at = base;
        // Add to `seen` the masses that haven't been counted yet.
        while let Some(new) = map.get(at) {
            at = new;
            if !seen.insert(new) {
                break;
            }
        }
        // I'm doing the summation of 0..*depths.get(base).unwrap() - 0..*depths.get(at).unwrap(),
        // which, given a = *depths.get(base).unwrap() and b = *depths.get(at).unwrap(), is equal to
        // a * (a - 1) / 2 - b * (b - 1) / 2, which is equivalent to
        // (a * (a - 1) - b * (b - 1)) / 2, which is that I add to `total`.
        let start_depth = match *depths.get(base).unwrap() {
            0 => 0,
            1 => 1,
            depth => depth * (depth - 1)
        };
        let end_depth = match *depths.get(at).unwrap() {
            0 => 0,
            1 => 1,
            depth => depth * (depth - 1)
        };
        total += (start_depth - end_depth) / 2;
    }
    println!("p1: {}", total);
    // This map will contain the nodes in the path from the parent of YOU to COM mapped to the
    // number of transfers it takes to get from the parent of YOU to any particular node.
    let mut you_path = HashMap::new();
    let mut at = map.get("YOU").unwrap();
    you_path.insert(at, 0);
    let mut num_transfers = 0;
    while let Some(parent) = map.get(at) {
        num_transfers += 1;
        you_path.insert(parent, num_transfers);
        at = parent;
    }
    at = map.get("SAN").unwrap();
    let mut san_transfers = 0;
    // Walk from the parent of SAN towards COM, and if the current node is in the path from YOU to
    // COM, we've found the closest node.
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

fn find_depths<'b, 'a: 'b>(
    parent_child: &'b HashMap<&'a str, Vec<&'a str>>,
    depth_map: &'b mut HashMap<&'a str, usize>,
    start: &'a str,
    depth: usize
) {
    depth_map.insert(start, depth);
    if let Some(children) = parent_child.get(start) {
        for child in children {
            find_depths(parent_child, depth_map, child, depth + 1)
        }
    }
}
