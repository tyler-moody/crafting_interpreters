use virtual_machine::bytecode::{Bytecode, Instruction, Value};

fn main() {
    let mut bytecode = Bytecode::new();
    bytecode.instruction(Instruction::Constant{value:Value::FloatingPoint(5.0)}, 0);
    bytecode.instruction(Instruction::Return, 0);
    println!("{}", bytecode);
}

