import unittest

from bytecode import *


class OpCodeTests(unittest.TestCase):
    def test_serialize_to_single_byte(self):
        opcode = OpCode.RETURN
        self.assertEqual(1, len(opcode.serialize()))


class ImmediateTests(unittest.TestCase):
    def test_serialize(self):
        expected = OpCode.IMMEDIATE.serialize() + struct.pack("f", 5.0)
        serialized = Immediate(Value(5.0)).serialize()
        self.assertEqual(expected, serialized)
        self.assertEqual(5, len(serialized))

    def test_disassemble(self):
        self.assertEqual("Immediate 5.0", Immediate(Value(5.0)).disassemble())


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
