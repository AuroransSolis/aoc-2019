mod lookup;

use lookup::LOOKUP;
use std::io::{stdin, BufRead};

fn main() {
    let mut p1 = 0;
    let mut p2 = 0;
    stdin().lock().split(b'\n').for_each(|line| {
        let mut starting_fuel = 0;
        let mut mul = 1;
        for c in line.unwrap().into_iter().rev() {
            starting_fuel += (c as u32 & 15) * mul;
            mul *= 10;
        }
        let starting_fuel = LOOKUP[starting_fuel as usize];
        p1 += starting_fuel;
        p2 += get_additional_fuel(starting_fuel);
    });
    println!("p1: {}\np2: {}", p1, p2);
    println!("{}", get_additional_fuel(654));
}

fn get_additional_fuel(fuel: u32) -> u32 {
    // If the amount of additional fuel would be 0 or negative, just return the current amount of
    // additional fuel.
    if fuel < 9 {
        fuel
    } else {
        // Otherwise, find out how much more we need.
        fuel + get_additional_fuel(LOOKUP[fuel as usize])
    }
}
