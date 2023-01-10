use crate::value::Value;
use std::fmt;

#[derive(Debug)]
pub enum Instruction {
    Return,
    Constant {value: Value},
    Negate,
    Add,
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Instruction::Return => {
                write!(f,"Return")
            }
            Instruction::Constant{value} => {
                write!(f, "Constant {}", value)
            }
            Instruction::Negate => {
                write!(f, "Negate")
            }
            Instruction::Add => {
                write!(f, "Add")
            }
        }
    }
}

pub struct Bytecode {
    code: Vec<Instruction>,
    line_numbers: Vec<usize>,
}

impl Bytecode {
    pub fn new() -> Self {
        Self {
            code: Vec::new(),
            line_numbers: Vec::new(),
        }
    }

    pub fn instruction(&mut self, instruction: Instruction, line: usize) {
        self.code.push(instruction);
        self.line_numbers.push(line);
    }

    pub fn len(&self) -> usize {
        self.code.len()
    }
}

impl fmt::Display for Bytecode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        assert!(self.code.len() == self.line_numbers.len());
        for (i, instruction) in self.code.iter().enumerate() {
            write!(f, "{}: {:4} {}\n", i, self.line_numbers[i], instruction)?
        }
        Ok(())
    }
}

impl std::ops::Index<usize> for Bytecode {
    type Output = Instruction;
    fn index(&self, index: usize) -> &Self::Output {
        self.code.index(index)
    }
}

#[test]
fn test_return() {
    let mut bytecode = Bytecode::new();
    bytecode.instruction(Instruction::Return, 0);
}

#[test]
fn test_constant() {
    let mut bytecode = Bytecode::new();
    bytecode.instruction(Instruction::Constant{value : Value::FloatingPoint(5.0)}, 0);
}
