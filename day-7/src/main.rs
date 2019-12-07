use std::fmt::{self, Display};
use std::io::{stdin, Read};

const OP: i64 = 0;
const P1: i64 = 1;
const P2: i64 = 2;
const P3: i64 = 3;

const ADD: i64 = 1;
const MUL: i64 = 2;
const GET: i64 = 3;
const PRT: i64 = 4;
const JIT: i64 = 5;
const JIF: i64 = 6;
const SLT: i64 = 7;
const SEQ: i64 = 8;
const BRK: i64 = 99;
const LAD: i64 = 0;
const LIM: i64 = 1;

#[derive(Debug)]
enum Step<'a> {
    Continue,
    Input(&'a mut i64),
    Output(i64),
    Break,
}

#[derive(Copy, Clone, Debug)]
enum StepError {
    InvalidOp(i64),
    InvalidMode(i64),
    OobLoad(i64),
    OobWrite(i64),
    NotEnoughInputs,
}

impl Display for StepError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            StepError::InvalidOp(op) => write!(f, "Encountered invalid opcode: {}", op),
            StepError::InvalidMode(mode) => {
                write!(f, "Encountered invalid addressing mode: {}", mode)
            }
            StepError::OobLoad(addr) => write!(f, "Tried to read out of bounds (addr: {})", addr),
            StepError::OobWrite(addr) => write!(f, "Tried to write out of bounds (addr: {})", addr),
            StepError::NotEnoughInputs => write!(f, "Not enough inputs!"),
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
        Program { mem, pc: 0 }
    }

    fn load(&self, mode: i64, part: i64) -> Result<i64, StepError> {
        match mode {
            LAD => match self.mem.get(self.pc + part as usize) {
                Some(idx) => match self.mem.get(*idx as usize) {
                    Some(val) => Ok(*val),
                    None => Err(StepError::OobLoad(*idx)),
                },
                None => Err(StepError::OobLoad(self.pc as i64 + part)),
            },
            LIM => match self.mem.get(self.pc + part as usize) {
                Some(val) => Ok(*val),
                None => Err(StepError::OobLoad(self.pc as i64 + part)),
            },
            _ => Err(StepError::InvalidMode(mode)),
        }
    }

    fn store(&mut self, idx: i64, val: i64) -> Result<(), StepError> {
        match self.mem.get_mut(idx as usize) {
            Some(v) => Ok(*v = val),
            None => Err(StepError::OobWrite(idx)),
        }
    }

    fn address(&mut self, idx: i64) -> Result<&mut i64, StepError> {
        match self.mem.get_mut(idx as usize) {
            Some(v) => Ok(v),
            None => Err(StepError::OobWrite(idx)),
        }
    }

    fn step(&mut self) -> Result<Step, StepError> {
        let mut complete = self.load(LIM, OP)?;
        let op = complete % 100;
        complete /= 100;
        let m1 = complete % 10;
        match op {
            op @ ADD | op @ MUL | op @ SLT | op @ SEQ => {
                complete /= 10;
                let m2 = complete % 10;
                let p1 = self.load(m1, P1)?;
                let p2 = self.load(m2, P2)?;
                let w = self.load(LIM, P3)?;
                self.store(
                    w,
                    match op {
                        ADD => p1 + p2,
                        MUL => p1 * p2,
                        SLT => (p1 < p2) as i64,
                        SEQ => (p1 == p2) as i64,
                        _ => unreachable!(),
                    },
                )?;
                self.pc += 4;
                Ok(Step::Continue)
            }
            op @ JIT | op @ JIF => {
                complete /= 10;
                let m2 = complete % 10;
                let p1 = self.load(m1, P1)?;
                let p2 = self.load(m2, P2)?;
                if match op {
                    JIT => p1 != 0,
                    JIF => p1 == 0,
                    _ => unreachable!(),
                } {
                    self.pc = p2 as usize;
                } else {
                    self.pc += 3;
                }
                Ok(Step::Continue)
            }
            GET => {
                let p1 = self.load(LIM, P1)?;
                self.pc += 2;
                let w = self.address(p1)?;
                Ok(Step::Input(w))
            }
            PRT => {
                let p1 = self.load(m1, P1)?;
                self.pc += 2;
                Ok(Step::Output(p1))
            }
            BRK => Ok(Step::Break),
            _ => Err(StepError::InvalidOp(op)),
        }
    }

    fn run(&mut self, input: &[i64]) -> Vec<i64> {
        let mut input_iter = input.iter();
        let mut outputs = Vec::new();
        loop {
            match self.step().unwrap() {
                Step::Continue => continue,
                Step::Input(w) => {
                    *w = *input_iter
                        .next()
                        .ok_or_else(|| &StepError::NotEnoughInputs)
                        .unwrap();
                },
                Step::Output(val) => outputs.push(val),
                Step::Break => break,
            }
        }
        outputs
    }

    fn step_to_output(&mut self, input: &[i64]) -> Option<i64> {
        let mut input_iter = input.iter();
        loop {
            match self.step().unwrap() {
                Step::Continue => continue,
                Step::Input(w) => {
                    *w = *input_iter
                        .next()
                        .ok_or_else(|| &StepError::NotEnoughInputs)
                        .unwrap();
                },
                Step::Output(val) => return Some(val),
                Step::Break => return None,
            }
        }
    }
}

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

/*
a1 → a2 → a3
⬑─────────┘
|-----------|