use std::io::{stdin, BufRead};
use std::process::exit;

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
    let memory = stdin().lock().split(b',').map(|section| {
        let mut section = section.unwrap();
        if section[section.len() - 1] == b'\n' {
            section.pop();
        }
        let mut val = 0;
        let mut mul = 1;
        for byte in section.into_iter().rev() {
            val += (byte as usize & 15) * mul;
            mul *= 10;
        }
        val
    }).collect::<Vec<usize>>();
    let program = Program::new(memory);
    part_1(program.clone());
    part_2(&program);
}

fn part_1(mut program: Program) {
    println!("{}", program.solve_for(12, 2));
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
