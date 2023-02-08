import enum
from parameterized import parameterized
import struct
import textwrap
from typing import Optional, Protocol

from value import Value


class OpCode(enum.IntEnum):
    RETURN = 1
    IMMEDIATE = 2
    NEGATE = 3
    ADD = 4
    SUBTRACT = 5
    MULTIPLY = 6
    DIVIDE = 7

    def serialize(self) -> bytes:
        opcode = struct.pack("i", self)
        truncated = opcode[:1]
        return truncated


class Instruction(Protocol):
    def serialize(self) -> bytes:
        ...

    def disassemble(self) -> str:
        ...


class Immediate:
    def __init__(self, value: Value):
        self.opcode = OpCode.IMMEDIATE
        self.value = value

    def serialize(self) -> bytes:
        return self.opcode.serialize() + self.value.serialize()

    def disassemble(self) -> str:
        return f"Immediate {self.value.disassemble()}"


class Return:
    OPCODE = OpCode.RETURN

    def serialize(self) -> bytes:
        return self.OPCODE.serialize()

    def disassemble(self) -> str:
        return "Return"


class Negate:
    OPCODE = OpCode.NEGATE

    def serialize(self) -> bytes:
        return self.OPCODE.serialize()

    def disassemble(self) -> str:
        return f"Negate"


class Add:
    OPCODE = OpCode.ADD

    def serialize(self) -> bytes:
        return self.OPCODE.serialize()

    def disassemble(self) -> str:
        return f"Add"


class Subtract:
    OPCODE = OpCode.SUBTRACT

    def serialize(self) -> bytes:
        return self.OPCODE.serialize()

    def disassemble(self) -> str:
        return f"Subtract"


class Multiply:
    OPCODE = OpCode.MULTIPLY

    def serialize(self) -> bytes:
        return self.OPCODE.serialize()

    def disassemble(self) -> str:
        return f"Multiply"


class Divide:
    OPCODE = OpCode.DIVIDE

    def serialize(self) -> bytes:
        return self.OPCODE.serialize()

    def disassemble(self) -> str:
        return f"Divide"


class ByteCode:
    def __init__(self):
        self.instructions = list()
        self.lines = list()

    def add_instruction(self, instruction: Instruction, line: int) -> None:
        self.instructions.append(instruction)
        self.lines.append(line)

    def serialize(self) -> bytes:
        result = bytes()
        for instruction in self.instructions:
            result += instruction.serialize()
        return result

    def disassemble(self) -> str:
        s = str()
        offset = 0
        for i in range(len(self.instructions)):
            instruction = self.instructions[i]
            if i > 0 and self.lines[i] == self.lines[i - 1]:
                line = "|"
            else:
                line = self.lines[i]
            s += f"{offset} {line:>4} 0x{' '.join([f'{byte:#0{4}x}'[2:] for byte in instruction.serialize()])} {instruction.disassemble()}\n"
            offset += len(instruction.serialize())
        return s
