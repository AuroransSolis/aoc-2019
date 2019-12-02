use std::io::{stdin, BufRead};
use std::process::exit;

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
    part_1(memory.clone());
    part_2(&memory);
}

fn part_1(mut memory: Vec<usize>) {
    memory[1] = 12;
    memory[2] = 2;
    let mut i = 0;
    while i < memory.len() {
        let op = match memory[i] {
            99 => break,
            1 => add,
            2 => mul,
            _ => {
                eprintln!("Encountered invalid opcode.");
                exit(0);
            }
        };
        let r1 = memory[i + 1];
        let r2 = memory[i + 2];
        let w = memory[i + 3];
        let res = op(memory[r1], memory[r2]);
        memory[w] = res;
        i += 4;
    }
    println!("{}", memory[0]);
}

fn part_2(memory: &Vec<usize>) {
    for i in 0..100 {
        for j in 0..100 {
            let mut tmp_memory = memory.clone();
            tmp_memory[1] = i;
            tmp_memory[2] = j;
            let mut n = 0;
            while n < tmp_memory.len() {
                let op = match tmp_memory[n] {
                    99 => break,
                    1 => add,
                    2 => mul,
                    _ => {
                        eprintln!("Encountered invalid opcode.");
                        exit(0);
                    }
                };
                let r1 = tmp_memory[n + 1];
                let r2 = tmp_memory[n + 2];
                let w = tmp_memory[n + 3];
                let res = op(tmp_memory[r1], tmp_memory[r2]);
                tmp_memory[w] = res;
                n += 4;
            }
            if tmp_memory[0] == 19690720 {
                println!("({}, {}) => {}", i, j, 100 * i + j);
                return;
            }
        }
    }
}

fn add(a: usize, b: usize) -> usize {
    a + b
}

fn mul(a: usize, b: usize) -> usize {
    a * b
}
