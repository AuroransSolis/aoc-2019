extern crate libc;

mod p1_lookup;
mod p2_lookup;

use libc::{lseek, SEEK_END, mmap, PROT_READ, MAP_SHARED};
use p1_lookup::P1_LOOKUP;
use p2_lookup::P2_LOOKUP;
use std::io::{stdin, BufRead};
use std::ptr::null_mut;
use std::slice::from_raw_parts;

fn main() {
    let mut p1 = 0;
    let mut p2 = 0;
    let len = unsafe { lseek(0, 0, SEEK_END) } as usize;
    let chars = unsafe { (mmap(null_mut(), len, PROT_READ, MAP_SHARED, 0, 0) as *const u8) };
    let chars = unsafe { from_raw_parts(chars, len) };
    let mut starting_fuel = 0;
    for &b in chars.iter() {
        if b == b'\n' {
            p1 += P1_LOOKUP[starting_fuel];
            p2 += P2_LOOKUP[starting_fuel];
            starting_fuel = 0;
        } else {
            starting_fuel *= 10;
            starting_fuel += b as usize & 15;
        }
    }
    println!("p1: {}\np2: {}", p1, p2);
}

/*fn get_additional_fuel(fuel: u32) -> u32 {
    // If the amount of additional fuel would be 0 or negative, just return the current amount of
    // additional fuel.
    if fuel < 9 {
        fuel
    } else {
        // Otherwise, find out how much more we need.
        fuel + get_additional_fuel(P1_LOOKUP[fuel as usize])
    }
}*/
