extern crate intcode_interpreter;
extern crate num_cpus;

use intcode_interpreter::Program;
use std::io::{stdin, Read};
use std::sync::{Arc, Mutex, atomic::{AtomicBool, Ordering}};
use std::thread;

fn main() {
    let mut memory = String::new();
    stdin().read_to_string(&mut memory).unwrap();
    let memory = memory
        .trim()
        .split(',')
        .map(|x| x.parse().unwrap())
        .collect::<Vec<i64>>();
    let program = Program::new(memory);
    part_1(program.clone());
    // part_2(&program);
    part_2_mt(&program);
}

fn part_1(mut program: Program) {
    println!("p1: {}", program.d2_solve_for(12, 2));
}

fn part_2(program: &Program) {
    for n in 0..100 {
        for v in 0..100 {
            let mut program = program.clone();
            if program.d2_solve_for(n, v) == 19690720 {
                println!("{}, {} => {}", n, v, 100 * n + v);
                return;
            }
        }
    }
}

fn part_2_mt(program: *const Program) {
    let iterator = Arc::new(Mutex::new((0..100).flat_map(|i| (0..100).map(move |j| (i, j)))));
    let done = Arc::new(AtomicBool::new(false));
    let program = program as usize;
    let threads = (0..num_cpus::get()).map(|_| {
        let iterator = iterator.clone();
        let done = done.clone();
        thread::spawn(move || {
            let iterator = iterator;
            let done = done;
            let program = unsafe { &*(program as *const Program) };
            loop {
                if done.load(Ordering::SeqCst) {
                    break;
                }
                let next = iterator.lock().unwrap().next();
                if let Some((n, v)) = next {
                    let mut program = program.clone();
                    if program.d2_solve_for(n, v) == 19690720 {
                        println!("p2: {}", 100 * n + v);
                        done.store(true, Ordering::SeqCst);
                        return;
                    }
                }
            }
        })
    }).collect::<Vec<_>>();
    threads.into_iter().for_each(|thread| thread.join().unwrap());
}
