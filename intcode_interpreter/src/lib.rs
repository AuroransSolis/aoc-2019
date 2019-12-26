use std::fmt::{self, Display};

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
const REL: i64 = 9;
const BRK: i64 = 99;
const LAD: i64 = 0;
const LIM: i64 = 1;
const LRL: i64 = 2;

#[derive(Debug)]
pub enum Step<'a> {
    Continue,
    Input(&'a mut i64),
    Output(i64),
    Break,
}

#[derive(Copy, Clone, Debug)]
pub enum StepError {
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
pub struct Program {
    mem: Vec<i64>,
    pc: usize,
    rel: usize,
}

impl Program {
    pub fn new(mem: Vec<i64>) -> Self {
        Program { mem, pc: 0, rel: 0 }
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
            LRL => match self.mem.get(self.pc + part as usize) {
                Some(offset) => match self.mem.get(self.rel + *offset as usize) {
                    Some(val) => Ok(*val),
                    None => Err(StepError::OobLoad(self.rel as i64 + *offset))
                },
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

    pub fn step(&mut self) -> Result<Step, StepError> {
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
            op @ PRT | op @ REL => {
                let p1 = self.load(m1, P1)?;
                self.pc += 2;
                match op {
                    PRT => Ok(Step::Output(p1)),
                    REL => {
                        self.rel = (self.rel as i64 + p1) as usize;
                        Ok(Step::Continue)
                    },
                    _ => unreachable!(),
                }
            }
            GET => {
                let p1 = self.load(LIM, P1)?;
                self.pc += 2;
                let w = self.address(p1)?;
                Ok(Step::Input(w))
            },
            BRK => Ok(Step::Break),
            _ => Err(StepError::InvalidOp(op)),
        }
    }

    pub fn run(&mut self, input: &[i64]) -> Vec<i64> {
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

    pub fn step_to_output(&mut self, input: &[i64]) -> Option<i64> {
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

    pub fn d2_solve_for(&mut self, verb: i64, noun: i64) -> i64 {
        self.mem[1] = verb;
        self.mem[2] = noun;
        let _ = self.run(&[]);
        self.mem[0]
    }
}
