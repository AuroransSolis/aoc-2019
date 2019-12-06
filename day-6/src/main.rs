use std::io::{stdin, Read};
use std::collections::HashMap;

fn main() {
    let mut input = String::new();
    stdin().lock().read_to_string(&mut input).unwrap();
    let mut map = HashMap::new();
    for line in input.lines() {
        let mut split = line.split(')');
        let orbited = split.next().unwrap();
        let orbiter = split.next().unwrap();
        map.entry(orbited).or_insert(vec![]).push(orbiter);
        map.entry(orbiter).or_insert(vec![]);
    }
    let mut total = map.len() - 1;
    for children in map.values() {
        if children.len() > 0 {
            for &child in children.iter() {
                total += get_num_indirects(&map, child);
            }
        }
    }
    println!("p1: {}", total);
    let mut map = HashMap::new();
    for line in input.lines() {
        let mut split = line.split(')');
        let orbited = split.next().unwrap();
        let orbiter = split.next().unwrap();
        map.entry(orbited).or_insert(vec![]).push(orbiter);
        map.entry(orbiter).or_insert(vec![]);
    }
    let mut num_transfers = 0;
    let mut you_base = get_parent(&map, "YOU");
    while !children_contains(&map, you_base, "SAN") && you_base != "COM" {
        let new_base = get_parent(&map, you_base);
        you_base = new_base;
        num_transfers += 1;
    }
    let mut san_base = get_parent(&map, "SAN");
    while san_base != you_base {
        let new_base = get_parent(&map, san_base);
        san_base = new_base;
        num_transfers += 1;
    }
    println!("p2: {}", num_transfers);
}

fn get_num_indirects(map: &HashMap<&str, Vec<&str>>, name: &str) -> usize {
    if let Some(children) = map.get(name) {
        children.len() + children.iter().map(|&child| get_num_indirects(map, child)).sum::<usize>()
    } else {
        0
    }
}

fn get_parent<'b, 'a: 'b>(map: &'b HashMap<&'a str, Vec<&'a str>>, name: &'a str) -> &'a str {
    map.iter()
        .find(|&(_, children)| children.contains(&name))
        .map(|(&name, _)| name)
        .expect("a")
}

fn children_contains(map: &HashMap<&str, Vec<&str>>, parent: &str, child: &str) -> bool {
    let children = map.get(parent).unwrap();
    if children.contains(&child) {
        true
    } else {
        children.iter().any(|&direct_child| children_contains(map, direct_child, child))
    }
}
