extern crate num_cpus;

use std::io::{stdin, Read};
use std::process::exit;
use std::sync::{Arc, Mutex, atomic::{AtomicBool, Ordering}};
use std::thread;

#[derive(Clone)]
struct Program {
    mem: Vec<usize>,
    pc: usize,
}

impl Program {
    fn new(mem: Vec<usize>) -> Self {
        Program {
            mem,
            pc: 0
        }
    }

    fn solve_for(&mut self, verb: usize, noun: usize) -> usize {
        self.mem[1] = verb;
        self.mem[2] = noun;
        while self.step() {}
        self.mem[0]
    }

    fn step(&mut self) -> bool {
        let res = match self.mem[self.pc] {
            1 => self.mem[self.mem[self.pc + 1]] + self.mem[self.mem[self.pc + 2]],
            2 => self.mem[self.mem[self.pc + 1]] * self.mem[self.mem[self.pc + 2]],
            99 => return false,
            _ => {
                eprintln!("Encountered invalid opcode.");
                exit(0);
            }
        };
        let write_addr = self.mem[self.pc + 3];
        self.mem[write_addr] = res;
        self.pc += 4;
        true
    }
}

fn main() {
    let mut memory = String::new();
    stdin().read_to_string(&mut memory).unwrap();
    let memory = memory
        .trim()
        .split(',')
        .map(|x| x.parse().unwrap())
        .collect::<Vec<usize>>();
    let program = Program::new(memory);
    part_1(program.clone());
    // part_2(&program);
    part_2_mt(&program);
}

fn part_1(mut program: Program) {
    println!("p1: {}", program.solve_for(12, 2));
}

fn part_2(program: &Program) {
    for n in 0..100 {
        for v in 0..100 {
            let mut program = program.clone();
            if program.solve_for(n, v) == 19690720 {
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
                    if program.solve_for(n, v) == 19690720 {
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
