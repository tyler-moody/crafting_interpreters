use std::fmt;

#[derive(Debug)]
pub enum OpCode {
    Return,
}

#[derive(Debug)]
pub struct Instruction {
    opcode: OpCode,
}

impl Instruction {
    pub fn new(opcode: OpCode) -> Self {
        Self {
            opcode
        }
    }
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self.opcode {
            OpCode::Return => {
                "Return"
            }
        };
        write!(f, "{}", s)
    }
}

pub struct Chunk {
    code: Vec<Instruction>,
}

impl Chunk {
    pub fn new() -> Self {
        Self {
            code: vec![],
        }
    }

    pub fn write(&mut self, instruction: Instruction) {
        self.code.push(instruction);
    }
}

impl fmt::Display for Chunk {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (i, instruction) in self.code.iter().enumerate() {
            write!(f, "{}: {}\n", i, instruction)?
        }
        Ok(())
    }
}

#[test]
fn test_push_opcode() {
    let mut chunk = Chunk::new();
    chunk.write(Instruction::new(OpCode::Return));
}
