use std::fs::read_to_string;
use std::io::stdin;
use std::process::exit;

#[derive(Copy, Clone, Debug)]
enum Mode {
    Addr(i64),
    Imm(i64),
}

const ADD: i64 = 1;
const MUL: i64 = 2;
const GET: i64 = 3;
const PRT: i64 = 4;
const JIT: i64 = 5;
const JIF: i64 = 6;
const SLT: i64 = 7;
const SEQ: i64 = 8;
const BRK: i64 = 99;

impl Mode {
    fn new(p: i64, v: i64) -> Self {
        match p {
            0 => Mode::Addr(v),
            1 => Mode::Imm(v),
            _ => panic!("Invalid mode encountered!"),
        }
    }

    fn unwrap(&self) -> i64 {
        match self {
            Mode::Addr(addr) => *addr,
            Mode::Imm(imm) => *imm,
        }
    }
}

#[derive(Clone)]
struct Program {
    mem: Vec<i64>,
    pc: usize,
}

impl Program {
    fn new(mem: Vec<i64>) -> Self {
        Program {
            mem,
            pc: 0
        }
    }

    fn step(&mut self) -> bool {
        let [op, p1, p2] = parse_instr(self.mem[self.pc]);
        match op {
            op @ ADD | op @ MUL | op @ SLT | op @ SEQ => {
                let v1 = Mode::new(p1, self.mem[self.pc + 1]);
                let v2 = Mode::new(p2, self.mem[self.pc + 2]);
                let v1 = match v1 {
                    Mode::Addr(addr) => self.mem[addr as usize],
                    Mode::Imm(imm) => imm,
                };
                let v2 = match v2 {
                    Mode::Addr(addr) => self.mem[addr as usize],
                    Mode::Imm(imm) => imm,
                };
                let write = self.mem[self.pc + 3] as usize;
                self.mem[write] = match op {
                    ADD => v1 + v2,
                    MUL => v1 * v2,
                    SLT => if v1 < v2 { 1 } else { 0 },
                    SEQ => if v1 == v2 { 1 } else { 0 },
                    _ => unreachable!(),
                };
                self.pc += 4;
            },
            op @ 5 | op @ 6 => {
                let v1 = Mode::new(p1, self.mem[self.pc + 1]);
                let v2 = Mode::new(p2, self.mem[self.pc + 2]);
                let v1 = match v1 {
                    Mode::Addr(addr) => self.mem[addr as usize],
                    Mode::Imm(imm) => imm,
                };
                let v2 = match v2 {
                    Mode::Addr(addr) => self.mem[addr as usize],
                    Mode::Imm(imm) => imm,
                };
                if match op {
                    JIT => v1 != 0,
                    JIF => v1 == 0,
                    _ => unreachable!(),
                } {
                    self.pc = v2 as usize;
                } else {
                    self.pc += 3;
                }
            }
            GET => {
                let v1 = Mode::new(p1, self.mem[self.pc + 1]);
                let mut input = String::new();
                stdin().read_line(&mut input).unwrap();
                let input = input.trim();
                let input = input.parse::<i64>().unwrap();
                self.mem[v1.unwrap() as usize] = input;
                self.pc += 2;
            },
            PRT => {
                let v1 = Mode::new(p1, self.mem[self.pc + 1]);
                let v1 = match v1 {
                    Mode::Addr(addr) => self.mem[addr as usize],
                    Mode::Imm(imm) => imm,
                };
                println!("{}", v1);
                self.pc += 2;
            },
            BRK => return false,
            _ => {
                eprintln!("Encountered invalid opcode: {}", op);
                exit(0);
            }
        };
        true
    }
}

fn main() {
    let memory = read_to_string("day-5/input.txt").unwrap();
    let memory = memory.trim();
    // let mut memory = String::new();
    // stdin().read_line(&mut memory).unwrap();
    // let memory = memory.trim();
    let memory = memory
        .trim()
        .split(',')
        .map(|x| x.parse::<i64>().expect("a"))
        .collect::<Vec<i64>>();
    let mut program = Program::new(memory);
    while program.step() {}
}

fn parse_instr(mut instr: i64) -> [i64; 3] {
    let op = instr % 100;
    instr /= 100;
    let p1 = instr % 10;
    instr /= 10;
    let p2 = instr % 10;
    [op, p1, p2]
}