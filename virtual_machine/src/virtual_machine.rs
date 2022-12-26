
use crate::bytecode::{Bytecode, Instruction};
use crate::value::Value;

#[derive(Debug, PartialEq)]
pub enum Error {
    NotImplemented,
    // TODO compile errors
    
    // runtime errors
    InvalidInstructionPointer,
    NoValue,
}

pub struct VirtualMachine {
}

struct Context {
    pub ip: usize,
    pub stack: Vec<Value>,
}

impl Context {
    fn new() -> Self {
        Self {
            ip: 0,
            stack: Vec::new(),
        }
    }
}

impl VirtualMachine {
    pub fn new() -> Self {
        Self {
        }
    }

    pub fn execute(self, code: Bytecode) -> Result<Value, Error> {
        let mut context = Context::new();
        loop {
            if context.ip >= code.len() {
                return Err(Error::InvalidInstructionPointer);
            }
            match code[context.ip] {
                Instruction::Constant {value} => {
                    context.stack.push(value);
                }
                Instruction::Return => {
                    return context.stack.pop().ok_or(Error::NoValue);
                }
                _ => {
                    return Err(Error::NotImplemented)
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bytecode::{Bytecode, Instruction};

    #[test]
    fn test_empty_bytecode() {
        let vm = VirtualMachine::new();
        assert_eq!(Err(Error::InvalidInstructionPointer), vm.execute(Bytecode::new()));
    }

    #[test]
    fn test_return_no_value() {
        let vm = VirtualMachine::new();
        let mut bytecode = Bytecode::new();
        bytecode.instruction(Instruction::Return, 0);
        assert_eq!(Err(Error::NoValue), vm.execute(bytecode));
    }

    #[test]
    fn test_execute() {
        let vm = VirtualMachine::new();
        let mut bytecode = Bytecode::new();
        bytecode.instruction(Instruction::Constant{value: Value::FloatingPoint(5.0)}, 0);
        bytecode.instruction(Instruction::Return, 0);
        assert_eq!(Ok(Value::FloatingPoint(5.0)), vm.execute(bytecode));
    }
}
