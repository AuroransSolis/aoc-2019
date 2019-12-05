extern crate libc;

mod p1_lookup;
mod p2_lookup;

use libc::{lseek, SEEK_END, mmap, PROT_READ, MAP_SHARED};
use p1_lookup::P1_LOOKUP;
use p2_lookup::P2_LOOKUP;
use std::ptr::null_mut;
use std::slice::from_raw_parts;

fn main() {
    let mut p1 = 0;
    let mut p2 = 0;
    let len = unsafe { lseek(0, 0, SEEK_END) } as usize;
    let chars = unsafe { (mmap(null_mut(), len, PROT_READ, MAP_SHARED, 0, 0) as *const u8) };
    let chars = unsafe { from_raw_parts(chars, len) };
    let mut mass = 0;
    for &b in chars.iter() {
        if b == b'\n' {
            p1 += unsafe { P1_LOOKUP.get_unchecked(mass) };
            p2 += unsafe { P2_LOOKUP.get_unchecked(mass) };
            mass = 0;
        } else {
            mass *= 10;
            mass += b as usize & 15;
        }
    }
    println!("p1: {}\np2: {}", p1, p2);
}
