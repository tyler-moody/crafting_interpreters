use virtual_machine::chunk::{Chunk, Instruction, OpCode};

fn main() {
    let mut chunk = Chunk::new();
    chunk.write(Instruction::new(OpCode::Return));
    chunk.write(Instruction::new(OpCode::Return));
    chunk.write(Instruction::new(OpCode::Return));
    println!("{}", chunk);
}

