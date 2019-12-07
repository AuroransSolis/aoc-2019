extern crate intcode_interpreter;

use intcode_interpreter::Program;
use std::io::{stdin, Read};

fn main() {
    let mut memory = String::new();
    stdin().lock().read_to_string(&mut memory).unwrap();
    let memory = memory.trim();
    let memory = memory
        .trim()
        .split(',')
        .map(|x| x.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();
    let mut p1 = Program::new(memory);
    let mut p2 = p1.clone();
    println!("p1:");
    p1.run(&[1]);
    println!("\np2:");
    p2.run(&[5]);
}