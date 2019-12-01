use std::fs::File;
use std::io::{stdin, BufRead, BufReader};

fn main() {
    let mut p1 = 0;
    let mut p2 = 0;
    stdin().lock().lines().for_each(|line| {
        let starting_fuel = (line.unwrap().parse::<usize>().unwrap() / 3).saturating_sub(2);
        p1 += starting_fuel;
        p2 += get_additional_fuel(starting_fuel);
    });
    /*BufReader::new(File::open("day-1/large_10m.txt").unwrap()).lines().for_each(|line| {
        let starting_fuel = (line.unwrap().parse::<usize>().unwrap() / 3).saturating_sub(2);
        p1 += starting_fuel;
        p2 += get_additional_fuel(starting_fuel);
    });*/
    println!("p1: {}\np2: {}", p1, p2);
}

fn get_additional_fuel(fuel: usize) -> usize {
    // If the amount of additional fuel would be 0 or negative, just return the current amount of
    // additional fuel.
    if fuel / 3 <= 2 {
        fuel
    } else {
        // Otherwise, find out how much more we need.
        fuel + get_additional_fuel(fuel / 3 - 2)
    }
}
