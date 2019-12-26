use std::env;
use std::fmt::{self, Display};
use std::fs::{self, File};
use std::io::{BufWriter, Write};
use std::path::Path;
use std::process::exit;

const ADD: &'static str = "ADD";
const MUL: &'static str = "MUL";
const GET: &'static str = "GET";
const PRT: &'static str = "PRT";
const JIT: &'static str = "JIT";
const JIF: &'static str = "JIF";
const SLT: &'static str = "SLT";
const SEQ: &'static str = "SEQ";
const BRK: &'static str = "BRK";

enum InstructionParseError {
    InvalidInstruction((usize, String)),
    BadFirstParameter((usize, String)),
    BadSecondParameter((usize, String)),
    BadThirdParameter((usize, String)),
    InvalidLiteral((usize, String)),
}

impl Display for InstructionParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            InstructionParseError::InvalidInstruction((line, instr)) => {
                write!(f, "line {} | invalid instruction: {}", line, instr)
            }
            InstructionParseError::BadFirstParameter((line, param)) => {
                write!(f, "line {} | bad first parameter: {}", line, param)
            }
            InstructionParseError::BadSecondParameter((line, param)) => {
                write!(f, "line {} | bad second parameter: {}", line, param)
            }
            InstructionParseError::BadThirdParameter((line, param)) => {
                write!(f, "line {} | bad third parameter: {}", line, param)
            }
            InstructionParseError::InvalidLiteral((line, literal)) => {
                write!(f, "line {} | invalid literal: {}", line, literal)
            }
        }
    }
}

enum Instruction {
    Add { m1: i64, m2: i64, p1: i64, p2: i64, p3: i64 },
    Mul { m1: i64, m2: i64, p1: i64, p2: i64, p3: i64 },
    GetI { p1: i64 },
    PrintI { m1: i64, p1: i64 },
    JumpNZ { m1: i64, m2: i64, p1: i64, p2: i64 },
    JumpZ { m1: i64, m2: i64, p1: i64, p2: i64 },
    SetLT { m1: i64, m2: i64, p1: i64, p2: i64, p3: i64 },
    SetEq { m1: i64, m2: i64, p1: i64, p2: i64, p3: i64 },
    Break,
    Other { v: i64 },
}

impl Instruction {
    fn new_3_param(line: usize, words: &[&str]) -> Result<Self, InstructionParseError> {
        let mut op_words = [""; 4];
        op_words.copy_from_slice(words);
        let [op, p1, p2, p3] = op_words;
        let (m1, p1) = if p1.starts_with('@') {
            // Split the @ from the rest of the number in case it's an address
            (0, p1.split_at(1).1)
        } else {
            // Otherwise leave it be
            (1, p1)
        };
        let p1 = p1
            .parse::<i64>()
            .map_err(|_| InstructionParseError::BadFirstParameter((line + 1, p1.to_string())))?;
        let (m2, p2) = if p2.starts_with('@') {
            // Split the @ from the rest of the number in case it's an address
            (0, p2.split_at(1).1)
        } else {
            // Otherwise leave it be
            (1, p2)
        };
        let p2 = p2
            .parse::<i64>()
            .map_err(|_| InstructionParseError::BadSecondParameter((line + 1, p2.to_string())))?;
        let p3 = p3
            .parse::<i64>()
            .map_err(|_| InstructionParseError::BadThirdParameter((line + 1, p3.to_string())))?;
        match op {
            ADD => Ok(Instruction::Add { m1, m2, p1, p2, p3 }),
            MUL => Ok(Instruction::Mul { m1, m2, p1, p2, p3 }),
            SLT => Ok(Instruction::SetLT { m1, m2, p1, p2, p3 }),
            SEQ => Ok(Instruction::SetEq { m1, m2, p1, p2, p3 }),
            _ => Err(InstructionParseError::InvalidInstruction((
                line + 1,
                op.to_string(),
            ))),
        }
    }

    fn new_2_param(line: usize, words: &[&str]) -> Result<Self, InstructionParseError> {
        let mut op_words = [""; 3];
        op_words.copy_from_slice(words);
        let [op, p1, p2] = op_words;
        let (m1, p1) = if p1.starts_with('@') {
            // Split the @ from the rest of the number in case it's an address
            (0, p1.split_at(1).1)
        } else {
            // Otherwise leave it be
            (1, p1)
        };
        let p1 = p1
            .parse::<i64>()
            .map_err(|_| InstructionParseError::BadFirstParameter((line + 1, p1.to_string())))?;
        let (m2, p2) = if p2.starts_with('@') {
            // Split the @ from the rest of the number in case it's an address
            (0, p2.split_at(1).1)
        } else {
            // Otherwise leave it be
            (1, p2)
        };
        let p2 = p2
            .parse::<i64>()
            .map_err(|_| InstructionParseError::BadSecondParameter((line + 1, p2.to_string())))?;
        match op {
            JIT => Ok(Instruction::JumpNZ { m1, m2, p1, p2 }),
            JIF => Ok(Instruction::JumpZ { m1, m2, p1, p2 }),
            _ => Err(InstructionParseError::InvalidInstruction((
                line + 1,
                op.to_string(),
            ))),
        }
    }

    fn new_1_param(line: usize, words: &[&str]) -> Result<Self, InstructionParseError> {
        let mut op_words = [""; 2];
        op_words.copy_from_slice(words);
        let [op, p1] = op_words;
        let (m1, p1) = if p1.starts_with('@') {
            // Split the @ from the rest of the number in case it's an address
            (0, p1.split_at(1).1)
        } else {
            // Otherwise leave it be
            (1, p1)
        };
        let p1 = p1.parse::<i64>().map_err(|_| {
            InstructionParseError::BadFirstParameter((line, p1.to_string()))
        })?;
        match op {
            GET => Ok(Instruction::GetI { p1 }),
            PRT => Ok(Instruction::PrintI { m1, p1 }),
            _ => Err(InstructionParseError::InvalidInstruction((line, op.to_string()))),
        }
    }

    fn opcode(&self) -> i64 {
        match self {
            Instruction::Add { .. } => 1,
            Instruction::Mul { .. } => 2,
            Instruction::GetI { .. } => 3,
            Instruction::PrintI { .. } => 4,
            Instruction::JumpNZ { .. } => 5,
            Instruction::JumpZ { .. } => 6,
            Instruction::SetLT { .. } => 7,
            Instruction::SetEq { .. } => 8,
            Instruction::Break => 99,
            Instruction::Other { v } => *v,
        }
    }
}

impl Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Instruction::Add { m1, m2, p1, p2, p3 }
            | Instruction::Mul { m1, m2, p1, p2, p3 }
            | Instruction::SetLT { m1, m2, p1, p2, p3 }
            | Instruction::SetEq { m1, m2, p1, p2, p3 } => {
                let complete = self.opcode() + m1 * 100 + m2 * 1000;
                write!(f, "{},{},{},{},", complete, p1, p2, p3)
            }
            Instruction::JumpZ { m1, m2, p1, p2 } | Instruction::JumpNZ { m1, m2, p1, p2 } => {
                let complete = self.opcode() + m1 * 100 + m2 * 100;
                write!(f, "{},{},{},", complete, p1, p2)
            }
            Instruction::GetI { p1 } => write!(f, "{},{},", 3, p1),
            Instruction::PrintI { m1, p1 } => {
                let complete = 4 + m1 * 100;
                write!(f, "{},{},", complete, p1)
            }
            Instruction::Break => write!(f, "99,"),
            Instruction::Other { v } => write!(f, "{},", v),
        }
    }
}

fn main() {
    let words = env::args().skip(1).collect::<Vec<String>>();
    let words = words.iter().map(|n| n.as_str()).collect::<Vec<&str>>();
    if words.len() == 1 && (words[0] == "h" || words[0] == "help") {
        println!("Usage: intcode-assembler if=/path/to/input/file of=/path/to/output/file");
    } else if words.len() < 2 {
        println!("Too few arguments!");
    } else if words.len() > 2 {
        println!("Too many arguments!");
    } else {
        let input = if words[0].starts_with("if=") {
            read_input_file(words[0])
        } else if words[1].starts_with("if=") {
            read_input_file(words[1])
        } else {
            println!("Missing input file parameter!");
            return;
        };
        let mut output = if words[0].starts_with("of=") {
            create_output_file(words[0])
        } else if words[1].starts_with("of=") {
            create_output_file(words[1])
        } else {
            println!("Missing output file parameter!");
            return;
        };
        let mut output_string = String::new();
        for (i, line) in input.lines().enumerate() {
            let line = line.split('#').next().unwrap();
            if line.len() > 0 {
                // Split into words
                let words = line.split_ascii_whitespace().collect::<Vec<&str>>();
                // Remove commas
                let words = words
                    .into_iter()
                    .map(|word| {
                        if word.ends_with(',') {
                            word.split_at(word.len() - 2).0
                        } else {
                            word
                        }
                    })
                    .collect::<Vec<&str>>();
                let instr = match words.len() {
                    4 => Instruction::new_3_param(i, &words),
                    3 => Instruction::new_2_param(i, &words),
                    2 => Instruction::new_1_param(i, &words),
                    1 => {
                        if words[0] == BRK {
                            Ok(Instruction::Break)
                        } else {
                            let v = match words[0].parse::<i64>().map_err(|_| {
                                InstructionParseError::InvalidLiteral((i + 1, words[0].to_string()))
                            }) {
                                Ok(v) => v,
                                Err(e) => {
                                    println!("{}", e);
                                    return;
                                }
                            };
                            Ok(Instruction::Other { v })
                        }
                    },
                    0 => continue,
                    _ => {
                        println!("line {} | line too long: {}", i + 1, line);
                        return;
                    }
                };
                match instr {
                    Ok(instruction) => output_string.push_str(&instruction.to_string()),
                    Err(e) => {
                        println!("{}", e);
                        return;
                    }
                }
            }
        }
        output_string.pop();
        if output.write_all(output_string.as_bytes()).is_err() {
            println!("Failed to write intcode to output file.");
        } else {
            println!("Successfully wrote intcode to file.");
        }
    }
}

fn read_input_file(s: &str) -> String {
    let (_, path) = s.split_at(3);
    if let Ok(contents) = fs::read_to_string(path) {
        contents
    } else if Path::new(path).exists() {
        println!("Failed to read contents of input file. May be lacking permissions.");
        exit(0);
    } else {
        println!("File does not exist!");
        exit(0);
    }
}

fn create_output_file(s: &str) -> BufWriter<File> {
    let (_, path) = s.split_at(3);
    if let Ok(file) = File::create(path) {
        BufWriter::new(file)
    } else {
        println!("Failed to create output file. May be lacking permissions.");
        exit(0);
    }
}
