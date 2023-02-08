import enum
import logging
import struct
import textwrap
import unittest
from parameterized import parameterized
from typing import Optional, Protocol

logging.basicConfig(level=logging.INFO)


class Value:
    def __init__(self, value: float):
        self.value = value

    def __eq__(self, other: object) -> bool:
        if not isinstance(other, Value):
            return NotImplemented
        return self.value == other.value

    def __add__(self, other: object) -> "Value":
        if not isinstance(other, Value):
            return NotImplemented
        return Value(self.value + other.value)

    def __sub__(self, other: object) -> "Value":
        if not isinstance(other, Value):
            return NotImplemented
        return Value(self.value - other.value)

    def __mul__(self, other: object) -> "Value":
        if not isinstance(other, Value):
            return NotImplemented
        return Value(self.value * other.value)

    def __truediv__(self, other: object) -> "Value":
        if not isinstance(other, Value):
            return NotImplemented
        return Value(self.value / other.value)

    def serialize(self) -> bytes:
        return struct.pack("f", self.value)

    def disassemble(self) -> str:
        return str(self.value)


class ValueTests(unittest.TestCase):
    def test_equality(self):
        self.assertEqual(Value(5.0), Value(5.0))
        self.assertNotEqual(Value(5.0), Value(6.0))

    def test_add(self):
        self.assertEqual(Value(10.0), Value(5.0) + Value(5.0))

    def test_subtract(self):
        self.assertEqual(Value(2.0), Value(7.0) - Value(5.0))

    def test_multiply(self):
        self.assertEqual(Value(25.0), Value(5.0) * Value(5.0))

    def test_divide(self):
        self.assertEqual(Value(2.0), Value(10.0) / Value(5.0))

    def test_serialize_size(self):
        self.assertEqual(4, len(Value(5.0).serialize()))


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


class OpCodeTests(unittest.TestCase):
    def test_serialize_to_single_byte(self):
        opcode = OpCode.RETURN
        self.assertEqual(1, len(opcode.serialize()))


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


class ImmediateTests(unittest.TestCase):
    def test_serialize(self):
        expected = OpCode.IMMEDIATE.serialize() + struct.pack("f", 5.0)
        serialized = Immediate(Value(5.0)).serialize()
        self.assertEqual(expected, serialized)
        self.assertEqual(5, len(serialized))

    def test_disassemble(self):
        self.assertEqual("Immediate 5.0", Immediate(Value(5.0)).disassemble())


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


class SimpleInstructionTests(unittest.TestCase):
    params = [
        [Return, "Return"],
        [Negate, "Negate"],
        [Add, "Add"],
        [Subtract, "Subtract"],
        [Multiply, "Multiply"],
        [Divide, "Divide"],
    ]

    @parameterized.expand(params)
    def test_serialize(self, cls, _):
        serialized = cls().serialize()
        self.assertEqual(cls.OPCODE.serialize(), serialized)

    @parameterized.expand(params)
    def test_disassemble(self, cls, text):
        self.assertEqual(text, cls().disassemble())


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


class ByteCodeTests(unittest.TestCase):
    def test_disassemble(self) -> None:
        code = ByteCode()
        code.add_instruction(Immediate(Value(5.0)), 1)
        code.add_instruction(Return(), 1)
        code.add_instruction(Return(), 2)
        self.assertEqual(
            textwrap.dedent(
                """\
            0    1 0x02 00 00 a0 40 Immediate 5.0
            5    | 0x01 Return
            6    2 0x01 Return
            """
            ),
            code.disassemble(),
        )


class ExecutionStatus(enum.Enum):
    OK = 0
    COMPILE_ERROR = 1
    RUNTIME_ERROR = 2
    NOT_IMPLEMENTED = 3


class ExecutionResult:
    def __init__(self, status: ExecutionStatus, value: Optional[Value] = None):
        self.status = status
        self.value = value

    def __eq__(self, other: object) -> bool:
        if not isinstance(other, ExecutionResult):
            return NotImplemented
        return (
            self.status == other.status
            and type(self.value) == type(other.value)
            and self.value == other.value
        )


class ExecutionResultTests(unittest.TestCase):
    def test_equality_status(self):
        lhs = ExecutionResult(ExecutionStatus.OK)
        rhs = ExecutionResult(ExecutionStatus.NOT_IMPLEMENTED)
        self.assertNotEqual(lhs, rhs)
        rhs.status = ExecutionStatus.OK
        self.assertEqual(lhs, rhs)

    def test_equality_value(self):
        lhs = ExecutionResult(ExecutionStatus.OK, value=Value(5.0))
        rhs = ExecutionResult(ExecutionStatus.OK)
        self.assertNotEqual(lhs, rhs)
        rhs.value = Value(5.0)
        self.assertEqual(lhs, rhs)


class Context:
    def __init__(self):
        self.stack: List[Value] = list()

    def push(self, value: Value) -> None:
        self.stack.append(value)

    def pop(self) -> Optional[Value]:
        if len(self.stack) < 1:
            return None
        return self.stack.pop()


class ContextTests(unittest.TestCase):
    def test_pop_no_pushed_values(self):
        context = Context()
        self.assertEqual(None, context.pop())

    def test_push_pop(self):
        context = Context()
        context.push(5)
        self.assertEqual(5, context.pop())
        self.assertEqual(None, context.pop())


class VirtualMachine:
    def execute(self, code: bytes) -> ExecutionResult:
        context = Context()
        while len(code) > 0:
            opcode, code = int.from_bytes(code[:1], "little"), code[1:]
            logging.debug(f"opcode {opcode}")
            if opcode == OpCode.RETURN:
                return ExecutionResult(status=ExecutionStatus.OK, value=context.pop())
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
                return ExecutionResult(status=ExecutionStatus.NOT_IMPLEMENTED)
        return ExecutionResult(status=ExecutionStatus.RUNTIME_ERROR)


class VirtualMachineTests(unittest.TestCase):
    def test_negate(self):
        code = ByteCode()
        code.add_instruction(Immediate(Value(5.0)), 1)
        code.add_instruction(Negate(), 1)
        code.add_instruction(Return(), 1)
        logging.debug(code.disassemble())
        vm = VirtualMachine()
        expected = ExecutionResult(status=ExecutionStatus.OK, value=Value(-5.0))
        actual = vm.execute(code.serialize())
        self.assertEqual(expected.status, actual.status)
        self.assertEqual(expected.value.value, actual.value.value)
        self.assertEqual(expected, actual)

    def test_add(self):
        code = ByteCode()
        code.add_instruction(Immediate(Value(5.0)), 1)
        code.add_instruction(Immediate(Value(5.0)), 1)
        code.add_instruction(Add(), 1)
        code.add_instruction(Return(), 1)
        logging.debug(code.disassemble())
        vm = VirtualMachine()
        expected = ExecutionResult(status=ExecutionStatus.OK, value=Value(10.0))
        actual = vm.execute(code.serialize())
        self.assertEqual(expected.status, actual.status)
        self.assertEqual(expected.value.value, actual.value.value)
        self.assertEqual(expected, actual)

    def test_subtract(self):
        code = ByteCode()
        code.add_instruction(Immediate(Value(5.0)), 1)
        code.add_instruction(Immediate(Value(3.0)), 1)
        code.add_instruction(Subtract(), 1)
        code.add_instruction(Return(), 1)
        logging.debug(code.disassemble())
        vm = VirtualMachine()
        expected = ExecutionResult(status=ExecutionStatus.OK, value=Value(2.0))
        actual = vm.execute(code.serialize())
        self.assertEqual(expected.status, actual.status)
        self.assertEqual(expected.value.value, actual.value.value)
        self.assertEqual(expected, actual)

    def test_multiply(self):
        code = ByteCode()
        code.add_instruction(Immediate(Value(5.0)), 1)
        code.add_instruction(Immediate(Value(5.0)), 1)
        code.add_instruction(Multiply(), 1)
        code.add_instruction(Return(), 1)
        logging.debug(code.disassemble())
        vm = VirtualMachine()
        expected = ExecutionResult(status=ExecutionStatus.OK, value=Value(25.0))
        actual = vm.execute(code.serialize())
        self.assertEqual(expected.status, actual.status)
        self.assertEqual(expected.value.value, actual.value.value)
        self.assertEqual(expected, actual)

    def test_divide(self):
        code = ByteCode()
        code.add_instruction(Immediate(Value(5.0)), 1)
        code.add_instruction(Immediate(Value(2.0)), 1)
        code.add_instruction(Divide(), 1)
        code.add_instruction(Return(), 1)
        logging.debug(code.disassemble())
        vm = VirtualMachine()
        expected = ExecutionResult(status=ExecutionStatus.OK, value=Value(2.5))
        actual = vm.execute(code.serialize())
        self.assertEqual(expected.status, actual.status)
        self.assertEqual(expected.value.value, actual.value.value)
        self.assertEqual(expected, actual)

    def test_no_return_is_runtime_error(self):
        code = ByteCode()
        code.add_instruction(Immediate(Value(5.0)), 1)
        vm = VirtualMachine()
        expected = ExecutionResult(status=ExecutionStatus.RUNTIME_ERROR)
        actual = vm.execute(code.serialize())
        self.assertEqual(expected.status, actual.status)
        self.assertEqual(expected, actual)


if __name__ == "__main__":
    unittest.main()
