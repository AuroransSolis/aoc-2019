mod p1_lookup;
mod p2_lookup;

use p1_lookup::P1_LOOKUP;
use p2_lookup::P2_LOOKUP;
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
        p1 += P1_LOOKUP[starting_fuel as usize];
        p2 += P2_LOOKUP[starting_fuel as usize];
    });
    println!("p1: {}\np2: {}", p1, p2);
}

fn get_additional_fuel(fuel: u32) -> u32 {
    // If the amount of additional fuel would be 0 or negative, just return the current amount of
    // additional fuel.
    if fuel < 9 {
        fuel
    } else {
        // Otherwise, find out how much more we need.
        fuel + get_additional_fuel(P1_LOOKUP[fuel as usize])
    }
}
