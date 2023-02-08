import enum
import logging
import struct
from typing import Optional

logging.basicConfig(level=logging.INFO)

from value import Value
from bytecode import (
    Add,
    ByteCode,
    Immediate,
    Divide,
    Multiply,
    Negate,
    OpCode,
    Return,
    Subtract,
)


class Status(enum.Enum):
    OK = 0
    COMPILE_ERROR = 1
    RUNTIME_ERROR = 2
    NOT_IMPLEMENTED = 3


class Result:
    def __init__(self, status: Status, value: Optional[Value] = None):
        self.status = status
        self.value = value

    def __eq__(self, other: object) -> bool:
        if not isinstance(other, Result):
            return NotImplemented
        return (
            self.status == other.status
            and type(self.value) == type(other.value)
            and self.value == other.value
        )


class Context:
    def __init__(self):
        self.stack: List[Value] = list()

    def push(self, value: Value) -> None:
        self.stack.append(value)

    def pop(self) -> Optional[Value]:
        if len(self.stack) < 1:
            return None
        return self.stack.pop()


class VirtualMachine:
    def execute(self, code: bytes) -> Result:
        context = Context()
        while len(code) > 0:
            opcode, code = int.from_bytes(code[:1], "little"), code[1:]
            logging.debug(f"opcode {opcode}")
            if opcode == OpCode.RETURN:
                return Result(status=Status.OK, value=context.pop())
            elif opcode == OpCode.IMMEDIATE:
                buf, code = code[:4], code[4:]
                value = struct.unpack("f", buf)[0]
                logging.debug(
                    f"immediate value {value} bytes {' '.join([f'{byte: #0{5}x}'[3:] for byte in buf])}"
                )
                context.push(Value(value))
            elif opcode == OpCode.NEGATE:
                value = context.pop()
                context.push(Value(-value.value))
            elif opcode == OpCode.ADD:
                rhs = context.pop()
                lhs = context.pop()
                context.push(lhs + rhs)
            elif opcode == OpCode.SUBTRACT:
                rhs = context.pop()
                lhs = context.pop()
                context.push(lhs - rhs)
            elif opcode == OpCode.MULTIPLY:
                rhs = context.pop()
                lhs = context.pop()
                context.push(lhs * rhs)
            elif opcode == OpCode.DIVIDE:
                rhs = context.pop()
                lhs = context.pop()
                context.push(lhs / rhs)
            else:
                return Result(status=Status.NOT_IMPLEMENTED)
        return Result(status=Status.RUNTIME_ERROR)
