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
    let p = Program::new(memory);
    part_1(&p);
    part_2(&p);
}

fn part_1(p: &Program) {
    let mut p1 = std::i64::MIN;
    let mut settings = [0, 1, 2, 3, 4];
    while next_lex_perm(&mut settings) {
        p1 = p1.max((0..5).fold(0, |acc, n| p.clone().run(&[settings[n], acc])[0]));
    }
    println!("p1: {}", p1);
}

fn part_2(p: &Program) {
    let mut p2 = std::i64::MIN;
    let mut settings = [5, 6, 7, 8, 9];
    while next_lex_perm(&mut settings) {
        let mut amps = [p.clone(), p.clone(), p.clone(), p.clone(), p.clone()];
        let mut input = amps.iter_mut()
            .enumerate()
            .fold(0, |acc, (i, p)| p.step_to_output(&[settings[i], acc]).unwrap());
        'run: loop {
            for amp in amps.iter_mut() {
                match amp.step_to_output(&[input]) {
                    Some(output) => input = output,
                    None => break 'run,
                }
            }
        }
        p2 = p2.max(input);
    }
    println!("p2: {}", p2);
}

fn next_lex_perm(set: &mut [i64]) -> bool {
    if set.len() == 0 {
        return false;
    }
    let mut i = set.len() - 1;
    while i > 0 && set[i - 1] >= set[i] {
        i -= 1;
    }
    if i == 0 {
        return false;
    }
    let mut j = set.len() - 1;
    while set[j] <= set[i - 1] {
        j -= 1;
    }
    set.swap(i - 1, j);
    set[i .. ].reverse();
    true
}
