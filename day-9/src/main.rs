extern crate intcode_interpreter;

use std::io::{stdin, Read};

fn main() {
    let mut input = String::new();
    stdin().lock().read_to_string(&mut input);
    let input = input.trim();
    part_1(input);
    part_2(input);
}

fn part_1(input: &str) {

}

fn part_2(input: &str) {

}
