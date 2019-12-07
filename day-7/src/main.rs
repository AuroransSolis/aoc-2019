extern crate intcode_interpreter;

use intcode_interpreter::Program;
use std::fmt::{self, Display};
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
    let p = Program::new(memory);
    let mut p1 = std::i64::MIN;
    for a in 0..5 {
        for b in (0..5).filter(|&b| b != a) {
            for c in (0..5).filter(|&c| ![a, b].contains(&c)) {
                for d in (0..5).filter(|&d| ![a, b, c].contains(&d)) {
                    for e in (0..5).filter(|&e| ![a, b, c, d].contains(&e)) {
                        let mut a1 = p.clone();
                        let mut a2 = a1.clone();
                        let mut a3 = a2.clone();
                        let mut a4 = a3.clone();
                        let mut a5 = a4.clone();
                        let output = a1.run(&[a, 0]);
                        let output = a2.run(&[b, output[0]]);
                        let output = a3.run(&[c, output[0]]);
                        let output = a4.run(&[d, output[0]]);
                        let output = a5.run(&[e, output[0]]);
                        p1 = p1.max(output[0]);
                    }
                }
            }
        }
    }
    println!("p1: {}", p1);
    let mut p2 = std::i64::MIN;
    for a in 5..10 {
        for b in (5..10).filter(|&b| b != a) {
            for c in (5..10).filter(|&c| ![a, b].contains(&c)) {
                for d in (5..10).filter(|&d| ![a, b, c].contains(&d)) {
                    for e in (5..10).filter(|&e| ![a, b, c, d].contains(&e)) {
                        let mut a1 = p.clone();
                        let mut a2 = a1.clone();
                        let mut a3 = a2.clone();
                        let mut a4 = a3.clone();
                        let mut a5 = a4.clone();
                        let output = a1.step_to_output(&[a, 0]).unwrap();
                        let output = a2.step_to_output(&[b, output]).unwrap();
                        let output = a3.step_to_output(&[c, output]).unwrap();
                        let output = a4.step_to_output(&[d, output]).unwrap();
                        let output = a5.step_to_output(&[e, output]).unwrap();
                        let mut amps = [a1, a2, a3, a4, a5];
                        let mut input = [output];
                        'run: loop {
                            for amp in amps.iter_mut() {
                                match amp.step_to_output(&input) {
                                    Some(output) => input = [output],
                                    None => break 'run,
                                }
                            }
                        }
                        p2 = p2.max(input[0]);
                    }
                }
            }
        }
    }
    println!("p2: {}", p2);
}
